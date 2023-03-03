// #![allow(unused_variables)]
#![allow(dead_code)]

use colored::*;
use issue::{Issue};

use maplit::hashmap;
use source_file::Span;

use std::{collections::{HashMap, HashSet}, rc::Rc};

mod tokens;
mod parser;
mod checker;
mod codegen;
mod source_file;
mod issue;

use checker as chk;

pub struct Module<'a> {
	builtins: HashMap<String, PropDecl>,
	imports: &'a HashMap<String, PathBuf>,
	props: &'a HashMap<String, PropDecl>,
	stack: Vec<HashMap<String, PropDecl>>,
	components: &'a HashMap<PathBuf, chk::Component>,
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
	pub default: Option<Expr>,
	pub span: Span,
}

impl PartialEq for PropDecl {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.prop_type == other.prop_type
	}
}

impl PropDecl {
	fn component(def: ComponentDef) -> Self {
		PropDecl {
			is_pub: true,
			name: def.name.clone(),
			prop_type: Type::Component(Rc::new(def)),
			default: None,
			span: Span::internal()
		}
	}
	fn module(def: ModuleDef) -> Self {
		PropDecl {
			is_pub: true,
			name: def.name.clone(),
			prop_type: Type::Module(Rc::new(def)),
			default: None,
			span: Span::internal()
		}
	}
	fn function<S: Into<String>>(name: S, args: Vec<Type>, ret: Box<Type>) -> Self {
		let name = name.into();
		PropDecl {
			is_pub: true,
			name: name.clone(),
			prop_type: Type::Function(args, ret),
			default: None,
			span: Span::internal()
		}
	}
}

#[derive(Debug, Clone)]
pub struct PropDef {
	prop_type: Type,
	children: Vec<String>,
}

#[derive(Debug)]
pub struct ComponentDef {
	name: String,
	props: HashMap<String, PropDef>,
	container: bool,
	child_rules: ChildRules,
}

impl PartialEq for ComponentDef {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

#[derive(Debug, Clone)]
pub struct ChildRules {
	pub any: bool,
	pub any_of: HashSet<Vec<String>>,
	pub one_of: HashSet<Vec<String>>,
}

impl ChildRules {
	fn any() -> Self {
		ChildRules { any: true, any_of: HashSet::new(), one_of: HashSet::new() }
	}

	fn none() -> Self {
		ChildRules { any: false, any_of: HashSet::new(), one_of: HashSet::new() }
	}

	fn any_of<S: AsRef<str>>(any_of: &[&[S]]) -> Self {
		ChildRules {
			any: false,
			any_of: any_of
				.iter()
				.map(|v| {
					v.iter()
						.map(|s| s.as_ref().to_owned())
						.collect::<Vec<String>>()
				})
				.collect(),
			one_of: HashSet::new(),
		}
	}

	fn is_container(&self) -> bool {
		self.any || !self.any_of.is_empty() || !self.one_of.is_empty()
	}
}

#[derive(Debug)]
pub struct ModuleDef {
	name: String,
	props: HashMap<String, PropDecl>,
}

impl PartialEq for ModuleDef {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

fn init_builtins() -> HashMap<String, PropDecl> {
	hashmap![
		"Rect".to_owned() => PropDecl::component(ComponentDef {
			name: "Rect".to_owned(),
			container: true,
			child_rules: ChildRules::any(),
			props: hashmap![
				"x1".to_owned() => PropDef { prop_type: Type::Length, children: vec![] },
				"y1".to_owned() => PropDef { prop_type: Type::Length, children: vec![] },
				"x2".to_owned() => PropDef { prop_type: Type::Length, children: vec![] },
				"y2".to_owned() => PropDef { prop_type: Type::Length, children: vec![] },
				"scaleToParent".to_owned() => PropDef {
					prop_type: Type::Float,
					children: vec![
						"x1".to_owned(),
						"y1".to_owned(),
						"x2".to_owned(),
						"y2".to_owned(),
					]
				},
				"fill".to_owned() => PropDef { prop_type: Type::Brush, children: vec![] },
			],
		}),
		"Layout".to_owned() => PropDecl::component(ComponentDef {
			name: "Layout".to_owned(),
			container: true,
			child_rules: ChildRules::any_of(&[&["Pane"]]),
			props: hashmap![
				"layout".to_owned() => PropDef { prop_type: Type::EnumLayout, children: vec![] },
				"padding".to_owned() => PropDef { prop_type: Type::Length, children: vec![] },
			],
		}),
		"Pane".to_owned() => PropDecl::component(ComponentDef {
			name: "Pane".to_owned(),
			container: true,
			child_rules: ChildRules::any(),
			props: hashmap![],
		}),
		"Text".to_owned() => PropDecl::component(ComponentDef {
			name: "Text".to_owned(),
			container: false,
			child_rules: ChildRules::none(),
			props: hashmap![
				"content".to_owned() => PropDef { prop_type: Type::String, children: vec![] },
			],
		}),
		"Brush".to_owned() => PropDecl::module(ModuleDef {
			name: "Brush".to_owned(),
			props: hashmap! [
				"rgb".to_owned() => PropDecl::function(
					"rgb",
					vec![Type::Float, Type::Float, Type::Float],
					Box::new(Type::Brush),
				),
			],
		}),
		"Math".to_owned() => PropDecl::module(ModuleDef {
			name: "Math".to_owned(),
			props: hashmap! [
				"random".to_owned() => PropDecl::function(
					"random",
					vec![],
					Box::new(Type::Float),
				),
			],
		}),
	]
}

impl <'a> Module<'a> {
	pub fn new(
		imports: &'a HashMap<String, PathBuf>,
		components: &'a HashMap<PathBuf, chk::Component>,
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

	fn push_scope(&mut self) {
		self.stack.push(HashMap::new());
	}

	fn pop_scope(&mut self) {
		self.stack.pop().unwrap();
	}

	fn declare<S: Into<String>>(&mut self, binding: S, t: &Type, span: &Span) -> Result<(), ()> {
		let binding = binding.into();
		if let Some((ctx, _)) = self.lookup(&[binding.clone()], span)? {
			let (message_part, fail) = match ctx {
				Ctx::Builtin => ("as a builtin item", true),
				Ctx::Component => ("as a component property", false),
				Ctx::Scope(0) => ("in this scope", true),
				Ctx::Scope(_) => ("in an outer scope", false),
			};
			let message = format!("binding `{}` already exists {}", binding, message_part);
			if fail {
				eprintln!("{}", Issue::error(message, span.clone()));
				return Err(());
			} else {
				eprintln!("{}", Issue::warning(message, span.clone()));
			}
		}
		let map = self.stack.last_mut().unwrap();
		map.insert(binding.clone(), PropDecl {
			is_pub: false,
			name: binding,
			prop_type:
			t.clone(),
			default: None,
			span: span.clone(),
		});
		Ok(())
	}

	fn lookup_in_map(&self, path: &[String], map: &HashMap<String, PropDecl>, span: &Span) -> Result<Option<Type>, ()> {
		let mut map = map;
		let mut it = path.iter().peekable();
		let mut segment = it.next().unwrap();
		let mut prop = if let Some(prop) = map.get(segment) {
			prop
		} else {
			return Ok(None)
		};

		loop {
			let t = &prop.prop_type;
			if it.peek().is_none() {
				return Ok(Some(t.clone()));
			} else if let Type::Object(next_map) = t {
				map = next_map;
			} else if let Type::Module(def) = t {
				map = &def.props;
			} else {
				let message = format!("`{}` (type `{}`) has no child properties", prop.name, prop.prop_type.name().cyan());
				eprintln!("{}", Issue::error(message, span.clone()));
				return Err(());
			}

			segment = it.next().unwrap();
			prop = if let Some(prop) = map.get(segment) {
				prop
			} else {
				let message = format!("property `{}` does not exist", segment);
				eprintln!("{}", Issue::error(message, span.clone()));
				return Err(());
			};
		}
	}

	fn lookup(&self, path: &[String], span: &Span) -> Result<Option<(Ctx, Type)>, ()> {
		if let Some(t) = self.lookup_in_map(path, &self.builtins, span)? {
			return Ok(Some((Ctx::Builtin, t)))
		}
		for (depth, map) in self.stack.iter().rev().enumerate() {
			if let Some(t) = self.lookup_in_map(path, map, span)? {
				return Ok(Some((Ctx::Scope(depth), t)))
			}
		}
		if let Some(t) = self.lookup_in_map(path, self.props, span)? {
			return Ok(Some((Ctx::Component, t)))
		}
		Ok(None)
	}

	fn get_component_def(&self, element: &parser::Element) -> Result<Rc<ComponentDef>, ()> {
		let prop = self.lookup(&element.path, &element.name_span)?;
		if let Some((_, t)) = prop {
			match t {
				Type::Component(def) => Ok(def.clone()),
				_ => {
					let message = format!("`{}` (type `{}`) is not a component", element.path.join("."), t.name().cyan());
					eprintln!("{}", Issue::error(message, element.name_span.clone()));
					return Err(());
				}
			}
		} else {
			let message = format!("`{}`: component not found", element.path.join("."));
			eprintln!("{}", Issue::error(message, element.name_span.clone()));
			return Err(());
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctx {
	Builtin,
	Component,
	Scope(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Int,
	Float,
	Length,
	Brush,
	String,
	Boolean,
	EnumLayout,
	Iter(Box<Type>),
	Object(HashMap<String, PropDecl>),
	Component(Rc<ComponentDef>),
	Module(Rc<ModuleDef>),
	Function(Vec<Type>, Box<Type>),
	Callback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumLayout {
	Row,
	Column,
}

impl Type {
	fn name(&self) -> String {
		match self {
			Type::Int => "Int".to_owned(),
			Type::Float => "Float".to_owned(),
			Type::Length => "Length".to_owned(),
			Type::Brush => "Brush".to_owned(),
			Type::String => "String".to_owned(),
			Type::Boolean => "Boolean".to_owned(),
			Type::Object(..) => "Object".to_owned(),
			Type::Iter(t) => format!("[{}]", t.name()),
			Type::Module(..) => "Module".to_owned(),
			Type::Component(def) => def.name.clone(),
			_ => unimplemented!("{:?}", self),
		}
	}

	fn iter_type(&self) -> Option<Type> {
		match self {
			Type::Int => Some(Type::Int),
			Type::Iter(t) => Some(*t.clone()),
			_ => None,
		}
	}

	fn is_iter(&self) -> bool {
		match self {
			Type::Iter(_) => true,
			_ => false,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Expr {
	value: ExprValue,
	span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprValue {
	Px(f64),
	Float(f64),
	Int(i32),
	Color(f64, f64, f64, f64),
	String(String),
	Enum(String, Option<String>),
	Boolean(bool),
	Object(HashMap<String, Expr>),
	Path(Vec<String>, Ctx),
	Coerce(Box<Expr>, Type),
	AsIter(Box<Expr>, Type),
	FunctionCall(Box<Expr>, Vec<Expr>),
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
	components: &'a mut HashMap<PathBuf, chk::Component>,
) -> Result<&'a chk::Component, ()> {
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
	let component = chk::check_component(&mut module, parse_tree)?;

	// let mut dir = path.parent().unwrap().to_path_buf();
	// dir.push("dist");
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
