// #![allow(unused_variables)]
#![allow(dead_code)]

use colored::*;
use issue::{Issue};

use maplit::hashmap;
use source_file::Span;

use std::collections::HashMap;

mod tokens;
mod parser;
mod elements;
mod codegen;
mod source_file;
mod issue;

use elements as el;

pub struct Module<'a> {
	builtins: HashMap<String, ComponentDef>,
	imports: &'a HashMap<String, PathBuf>,
	props: &'a HashMap<String, PropDecl>,
	stack: Vec<HashMap<String, Type>>,
	components: &'a HashMap<PathBuf, el::Component>,
}

#[derive(Debug)]
pub struct Import {
	pub path: PathBuf,
	pub alias: Option<String>,
	pub span: Span,
}

#[derive(Debug, Clone)]
pub struct PropDecl {
	pub is_pub: bool,
	pub name: String,
	pub prop_type: Type,
	pub default: Option<Value>,
	pub span: Span,
}

struct ComponentDef {
	props: HashMap<String, Type>,
}

fn init_builtins() -> HashMap<String, ComponentDef> {
	hashmap![
		"Rect".to_owned() => ComponentDef {
			props: hashmap![
				"x1".to_owned() => Type::Length,
				"y1".to_owned() => Type::Length,
				"x2".to_owned() => Type::Length,
				"y2".to_owned() => Type::Length,
				"fill".to_owned() => Type::Brush,
			],
		},
	]
}

impl <'a> Module<'a> {
	pub fn new(
		imports: &'a HashMap<String, PathBuf>,
		components: &'a HashMap<PathBuf, el::Component>,
		props: &'a HashMap<String, PropDecl>,
	) -> Self {
		Self {
			imports,
			props,
			components,
			stack: Vec::new(),
			builtins: init_builtins(),
		}
	}

	fn get_component_def(&self, element: &parser::Element) -> Result<&ComponentDef, ()> {
		let path = &element.path;
		assert!(path.len() == 1);
		self.builtins.get(&path[0]).ok_or_else(|| {
			let message = format!("{}: component not found", path[0]);
			eprintln!("{}", Issue::error(message, element.name_span.clone()));
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctx {
	Component,
}

#[derive(Debug, Clone)]
pub enum Expr {
	Path(Vec<String>, Ctx),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
	Length,
	Brush,
	String,
	Boolean,
}

impl Type {
	fn name(&self) -> String {
		match self {
			Type::Length => "Length".to_owned(),
			Type::Brush => "Brush".to_owned(),
			Type::String => "String".to_owned(),
			Type::Boolean => "Boolean".to_owned(),
			_ => unimplemented!(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct ValueExpr {
	value: Value,
	span: Span,
}

#[derive(Debug, Clone)]
pub enum Value {
	Px(f64),
	Color(f64, f64, f64, f64),
	String(String),
	Boolean(bool),
	Expr(Expr),
	Object(HashMap<String, ValueExpr>),
}

use std::{ fs, process, path::PathBuf };

fn resolve_ui_import<'a>(
	path: &PathBuf,
	components: &'a mut HashMap<PathBuf, parser::Component>,
	span: Option<&Span>,
) -> Result<(String, PathBuf), ()> {
	let pathbuf = if let Ok(path) = fs::canonicalize(path) {
		Some(path)
	} else if let Ok(path) = fs::canonicalize(path.with_extension("ui")) {
		Some(path)
	} else {
		None
	};
	let pathbuf = if pathbuf.is_none() || !pathbuf.as_ref().unwrap().is_file() {
		let message = format!("invalid path specified: {}", path.display());
		if let Some(span) = span {
			eprintln!("{}", Issue::error(message, span.clone()));
		} else {
			eprintln!("{}", message.bold().red());
		}
		return Err(());
	} else {
		pathbuf.unwrap()
	};

	if let Some(component) = components.get(&pathbuf) {
		return Ok((component.name.clone(), pathbuf));
	}

	let mut component = parser::parse(&pathbuf)?;
	let mut import_decls = component.import_decls;
	let name = component.name.clone();
	component.import_decls = Vec::new();

	// inserting the component now results in some awkwardness, but it is required in case of recursive imports
	components.insert(pathbuf.clone(), component);
	
	while let Some(mut import) = import_decls.pop() {
		let alias = import.alias.clone();
		if import.path.is_relative() {
			let mut pathbuf = pathbuf.parent().unwrap().to_path_buf();
			pathbuf.push(import.path);
			import.path = pathbuf;
		}
		let (name, path) = resolve_ui_import(&import.path, components, Some(&import.span))?;

		// get the same component we just inserted above and update its imports map
		components.get_mut(&pathbuf).unwrap().imports_map.insert(alias.unwrap_or(name), path);
	}

	Ok((name, pathbuf))
}

fn load_ui_component<'a>(
	path: &str,
	parse_trees: &mut HashMap<PathBuf, parser::Component>
) -> Result<PathBuf, ()> {
	let (_, path) = resolve_ui_import(&path.into(), parse_trees, None)?;
	Ok(path)
}

fn build_impl<'a>(
	path: &PathBuf,
	parse_trees: &HashMap<PathBuf, parser::Component>,
	components: &'a mut HashMap<PathBuf, el::Component>,
) -> Result<&'a el::Component, ()> {
	use parser::CompileStatus;
	
	let parse_tree = parse_trees.get(path).unwrap();
	match parse_tree.status.get() {
		CompileStatus::Ready => {},
		CompileStatus::Building => {
			eprintln!("encountered recursive import: '{}'", path.display());
			return Err(());
		},
		CompileStatus::Done => { return Ok(components.get(path).unwrap()); },
	}
	parse_tree.status.set(CompileStatus::Building);
	
	for (_, path) in parse_tree.imports_map.iter() {
		build_impl(path, parse_trees, components)?;
	}

	let mut module = Module::new(&parse_tree.imports_map, components, &parse_tree.props);
	let component = el::check_component(&mut module, parse_tree)?;

	let dir = PathBuf::from("./dist");

	codegen::generate(&component, dir);

	parse_tree.status.set(CompileStatus::Done);
	components.insert(path.clone(), component);
	Ok(components.get(path).unwrap())
}

pub fn build(path: &str) -> Result<Vec<PathBuf>, ()> {
	let mut parse_trees = HashMap::new();
	let path = load_ui_component(&path, &mut parse_trees)?;

	let mut components = HashMap::new();
	if let Err(_) = build_impl(&path, &parse_trees, &mut components) {
		eprintln!("{}", "Compliation failed.".bold().red());
		Err(())
	} else {
		Ok(components.into_iter().map(|(k,_)|k).collect())
	}
}

pub fn watch(path: &str) {
	use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
	use std::sync::mpsc::channel;
	use std::time::Duration;

	let (tx, rx) = channel();
	let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();
	let mut prev_paths = Vec::new();

	let mut build_once = || {
		match build(path) {
			Ok(paths) => {
				for path in prev_paths.iter() {
					watcher.unwatch(path).unwrap();
				}
				for path in paths.iter() {
					watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
					println!("{}: {}", "watching".blue().bold(), format!("{}", path.display()).bold());
				}
				prev_paths = paths;
				true
			},
			Err(_) => {
				if prev_paths.len() == 0 {
					watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
				}
				println!("{}", "waiting for changes...".dimmed().bold());
				false
			}
		}
	};
	
	build_once();

	loop {
		match rx.recv() {
		   Ok(DebouncedEvent::Write(_)) => {
			   println!("{}", "rebuilding...".dimmed().bold());
			   build_once();
		   },
		   Err(e) => {
			   eprintln!("internal error: {:?}", e);
			   process::exit(1);
		   },
		   _ => {},
		}
	}
}
