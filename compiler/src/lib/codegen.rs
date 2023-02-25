use std::{path::PathBuf, fmt};
use std::fs::File;
use std::io::Write as IoWrite;

use colored::*;
use convert_case::{Casing, Case};

use crate::{Type, checker::Content, ExprValue, Ctx, chk::CheckedExpr};

use super::checker::{Component, Element};

fn type_to_js(prop_type: &Type) -> String {
	match prop_type {
		Type::Int => "Dom.Int".to_owned(),
		Type::Float => "Dom.Float".to_owned(),
		Type::Length => "Dom.Length".to_owned(),
		Type::Brush => "Dom.Brush".to_owned(),
		Type::String => "Dom.String".to_owned(),
		Type::Boolean => "Dom.Boolean".to_owned(),
		_ => unimplemented!(),
	}
}

#[derive(Debug, Clone)]
enum StaticValue {
	Px(f64),
	Float(f64),
	Int(i64),
	Color(f64, f64, f64, f64),
	String(String),
	Boolean(bool),
	BuiltinFunction(BuiltinFunction),
}

#[derive(Clone)]
struct BuiltinFunction(fn(&[StaticValue]) -> StaticValue);

impl fmt::Debug for BuiltinFunction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<BuiltinFunction>")
	}
}

impl ExprValue {
	fn try_into_static(&self) -> Option<StaticValue> {
		match self {
			ExprValue::Px(value) => Some(StaticValue::Px(*value)),
			ExprValue::Float(value) => Some(StaticValue::Float(*value)),
			ExprValue::Int(value) => Some(StaticValue::Int(*value)),
			ExprValue::Color(r,g,b,a) => Some(StaticValue::Color(*r,*g,*b,*a)),
			ExprValue::String(value) => Some(StaticValue::String(value.clone())),
			ExprValue::Boolean(value) => Some(StaticValue::Boolean(*value)),
			ExprValue::Object(..) => unimplemented!(),
			ExprValue::Path(..) => None,
			ExprValue::Coerce(..) => None,
			ExprValue::Enum(..) => None,
			ExprValue::FunctionCall(expr, args) => {
				let expr = expr.value.try_into_static();
				let args: Vec<_> = args.iter().map(|e| e.value.try_into_static()).collect();
				if expr.is_none() || args.iter().find(|e| e.is_none()).is_some() {
					return None;
				}
				let function = match expr.unwrap() {
					StaticValue::BuiltinFunction(function) => function.0,
					_ => unreachable!(),
				};
				let args: Vec<_> = args.into_iter().map(|e| e.unwrap()).collect();
				Some(function(&args))
			},
		}
	}
}

fn static_value_to_js(value: &StaticValue) -> String {
	match value {
		StaticValue::Px(n) => format!("Dom.Length.px({n})"),
		StaticValue::Float(n) => format!("Dom.Float.from({n})"),
		StaticValue::Int(n) => format!("Dom.Int.from({n})"),
		StaticValue::Color(r, g, b, a) => format!("Dom.Brush.rgba({r},{g},{b},{a})"),
		StaticValue::String(n) => {
			let n = format!("\"{}\"", n.replace("\n", "\\n").replace("\t", "\\t"));
			format!("Dom.String.from({n})")
		},
		StaticValue::Boolean(n) => format!("Dom.Boolean.{}", if *n { "true" } else { "false" }),
		StaticValue::BuiltinFunction(..) => unreachable!(),
	}
}

fn generate_property_assignment(name: &str, expr: &CheckedExpr, is_preset: bool) -> String {
	let constant_value = if let Some(value) = expr.expr.value.try_into_static() {
		Some(static_value_to_js(&value))
	} else if expr.bindings.len() == 0 {
		Some(expr_to_js(&expr.expr.value))
	} else {
		None
	};

	if let Some(constant_value) = constant_value {
		if is_preset {
			format!("e.{name}.set({constant_value}); ")
		} else {
			format!("e.props.{name} = {constant_value}; ")
		}
	} else {
		let rendered_expr = expr_to_js(&expr.expr.value);
		let required_bindings: String = expr.bindings.iter().map(|e| format!("this.bindings.{e},")).collect();
		let received_props = expr.bindings.join(",");
		let received_props = if received_props == "" { "()".into() } else { format!("([{received_props}])") };
		let binding = if is_preset {
			format!("e.{name}")
		} else {
			format!("e.bindings.{name}")
		};
		format!("{binding}.connect([{required_bindings}], {received_props} => {rendered_expr}); ")
	}
}

fn expr_to_js(value: &ExprValue) -> String {
	match value {
		ExprValue::Enum(name, enum_name) => {
			let name = name.to_case(Case::UpperCamel);
			format!("Dom.Enum.{}.{}", enum_name.clone().unwrap(), name)
		}
		ExprValue::Path(path, ctx) => {
			let ctx = match ctx {
				Ctx::Scope(_) => "",
				Ctx::Component => "",
				Ctx::Builtin => "Dom.Builtins.",
			};
			format!("{}{}", ctx, path.join("."))
		},
		ExprValue::Coerce(expr, coerce_type) => {
			format!("{}.coerce({})", type_to_js(coerce_type), expr_to_js(&expr.value))
		},
		ExprValue::FunctionCall(expr, args) => {
			let args = args.iter().map(|e| expr_to_js(&e.value)).collect::<Vec<String>>().join(", ");
			format!("{}({})", expr_to_js(&expr.value), args)
		},
		_ => unimplemented!("{:?}", value),
	}
}

fn generate_element(element: &Element) -> String {
	generate_element_impl(element, false)
}

fn generate_element_impl(element: &Element, skip_repeater: bool) -> String {
	if !skip_repeater && element.repeater.is_some() {
		let repeater = element.repeater.as_ref().unwrap();
		let item_type = type_to_js(&repeater.item_type);
		let index  = repeater.index.clone().unwrap_or("_$unused_index".into());
		let item  = repeater.item.clone().unwrap_or("_$unused_item".into());
		let collection = generate_property_assignment("collection", &repeater.collection, false);
		return format!(
			"(() => {{
				let e = dom.Repeater<{item_type}, Dom.{}>
				({item_type}, ({index}, {item}) => {{
					return [{}];
				}});
				{collection}
				return e;
			}})()",
			repeater.root_type,
			generate_element_impl(element, true)
		);
	}

	let props: String = element.props.iter()
		.map(|(k, v)| generate_property_assignment(k, v, false)).collect();
	let presets: String = element.presets.iter()
		.map(|(k, v)| generate_property_assignment(k, v, true)).collect();
	let children: String = element.children.iter().map(|child| {
		match child {
			Content::Element(e) => {
				format!("e.children.append({});", generate_element(e))
			},
			_ => unimplemented!(),
		}
	}).collect();
	format!(
		"(() => {{
			let e = dom.{}();
			{props}{presets}{children}
			return e;
		}})()",
		element.tag.path[0])
}

pub fn generate<P: Into<PathBuf>>(
	component: &Component,
	path: P,
) {
	let mut ctx = CodeGenCtx::new(&component.name, path);

	let component_name = &component.name;
	let model_props: String = component.props
		.iter()
		.map(|prop| { format!("{}: new Dom.Binding({}),", prop.name, type_to_js(&prop.prop_type)) })
		.collect();
	let model = format!("#model = new Dom.Model({{{model_props}}});");
	let root_class = format!("{}", component.root.tag.path[0]);
	let root = format!("readonly root: Dom.{root_class};");
	let events = format!("readonly events: Dom.{root_class}['events'];");
	let root_setup = format!("this.root = {}", generate_element(&component.root));
	let constructor_body = format!("super(); {root_setup}; this.events = this.root.events; ");
	let constructor = format!("constructor(dom: Dom.Dom) {{{constructor_body}}}");
	let impls = format!("{} {} {} {} {}",
		"get props() { return this.#model.props; }",
		"get bindings() { return this.#model.bindings; }",
		"getRoots() { return [this.root]; }",
		"get children() { return this.root.children }",
		"inject(deps: { [key: string]: any }) { this.root.inject(deps); }",
	);
	let class_body = format!("{model} {root} {events} {constructor} {impls}");

	writeln!(ctx.file, "import * as Dom from '../dom';").unwrap();
	writeln!(
		ctx.file,
		"export default class {component_name} extends Dom.Component<Dom.{root_class}> {{{class_body}}}",
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
