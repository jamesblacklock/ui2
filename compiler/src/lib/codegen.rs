use std::path::PathBuf;
use std::fs::File;
use std::io::Write as IoWrite;

use colored::*;
use convert_case::{Casing, Case};

use crate::{Type, Value, elements::Content};

use super::elements::{Component, Element};

fn type_to_js(prop_type: &Type) -> String {
	let t = match prop_type {
		Type::Length => "Dom.Length",
		Type::Brush => "Dom.Brush",
		_ => unimplemented!(),
	};

	format!("new Dom.Binding({})", t)
}

fn expr_to_js(expr: &Expr) -> String {
	match expr {
		Expr::Path(p, ctx) => {
			assert!(p.len() == 1);
			format!("this.props")
		}
	}
}

fn value_to_js(value: &Value) -> String {
	match value {
		Value::Px(n) => format!("Dom.Length.px({n})"),
		Value::Color(r, g, b, a) => format!("Dom.Brush.rgba({r},{g},{b},{a})"),
		Value::Expr(e) => unreachable!(),
		_ => unimplemented!(),
	}
}

pub fn generate_property_assignment(name: &String, expr: &ValueExpr) -> String {
	if let Value::Expr(e) = &expr.value {
		let some_prop =	match &expr.value {
			Value::Expr(Expr::Path(p, _)) => p[0].clone(),
			_ => unimplemented!()
		};
		format!("e.bindings.{name}.connect([this.bindings.{some_prop}], ([{some_prop}]) => {some_prop});")
	} else {
		let value_js = value_to_js(&expr.value);
		format!("e.props.{name} = {value_js}; ")
	}
}

pub fn generate_element<S: Into<String>>(js_target: S, element: &Element) -> String {
	let js_target = js_target.into();
	let props: String = element.props.iter()
		.map(|(k, v)| generate_property_assignment(k, v)).collect();
	let children: String = element.children.iter().map(|child| {
		match child {
			Content::Element(e) => {
				format!("var c = dom.{}(); {} e.children.append(c);", e.tag.path[0], generate_element("c", e))
			},
			_ => String::new(),
		}
	}).collect();
	format!("(e => {{ {props}{children} }})({js_target});")
}

pub fn generate<P: Into<PathBuf>>(
	component: &Component,
	path: P,
) {
	let mut ctx = CodeGenCtx::new(&component.name, path);

	let component_name = &component.name;
	let model_props: String = component.props
		.iter()
		.map(|prop| { format!("{}: {},", prop.name, type_to_js(&prop.prop_type)) })
		.collect();
	let model = format!("#model = new Dom.Model({{{model_props}}});");
	let root_class = format!("{}", component.root.tag.path[0]);
	let root = format!("readonly root: Dom.{root_class};");
	let events = format!("readonly events: Dom.{root_class}['events'];");
	let root_setup = generate_element("this.root", &component.root);
	let constructor_body = format!("this.root = dom.{root_class}(); this.events = this.root.events; {root_setup}");
	let constructor = format!("constructor(dom: Dom.Dom) {{{constructor_body}}}");
	let impls = format!("{} {} {} {}",
		"get props() { return this.#model.props; }",
		"get bindings() { return this.#model.bindings; }",
		"getRoot() { return this.root; }",
		"get children() { return this.root.children }",
	);
	let class_body = format!("{model} {root} {events} {constructor} {impls}");

	writeln!(ctx.file, "import * as Dom from '../dom';").unwrap();
	writeln!(
		ctx.file,
		"export default class {component_name} implements Dom.Component<Dom.Element>, Dom.Container {{{class_body}}}",
	).unwrap();

	ctx.finalize();
}

pub struct CodeGenCtx {
	file: File,
	file_name: String,
	dir: PathBuf,
	tempname: PathBuf,
	index: usize,
}

impl CodeGenCtx {
	pub fn new<S: Into<String>, P: Into<PathBuf>>(name: S, dir: P) -> CodeGenCtx {
		let file_name = name.into().to_case(Case::Kebab);
		let dir = dir.into();
		std::fs::create_dir_all(&dir).unwrap();
		let mut tempname = dir.clone();
		let timestamp = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
			.as_millis();
		tempname.push(format!("{}.ts.{}", file_name, timestamp));
		let file = File::create(&tempname).unwrap();
		CodeGenCtx {
			file,
			file_name,
			index: 0,
			dir,
			tempname,
		}
	}

	fn finalize(self) {
		std::mem::drop(self.file);
		let mut path = self.dir;
		path.push(format!("{}.ts", self.file_name));
		if path.is_file() {
			if std::fs::remove_file(&path).is_err() {
				eprintln!("unable to replace file: {}", path.display());
				return;
			}
		}
		if std::fs::rename(&self.tempname, &path).is_err() {
			eprintln!("unable to rename file: {}", self.tempname.display());
			return;
		}
		println!("{}: {}", "built component".green().bold(), format!("{}", path.display()).bold());
	}
}
