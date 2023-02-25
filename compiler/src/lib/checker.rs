use colored::*;
use maplit::hashset;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use crate::issue::Issue;
use crate::source_file::Span;
use crate::{ChildRules, ComponentDef, Ctx, Expr, ExprValue};

use super::{
	parser::Component as ParserComponent,
	parser::Content as ParserContent,
	parser::Element as ParserElement,
	Module,
	PropDecl,
	Type,
};

pub use super::parser::Children;

#[derive(Debug)]
pub enum Content {
	Element(Element),
	Children(Children),
}

#[derive(Debug)]
pub struct ElementTag {
	pub path: Vec<String>,
	pub import_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Element {
	pub tag: ElementTag,
	pub data: Option<CheckedExpr>,
	pub condition: Option<CheckedExpr>,
	pub repeater: Option<CheckedRepeater>,
	pub props: HashMap<String, CheckedExpr>,
	pub presets: HashMap<String, CheckedExpr>,
	pub children: Vec<Content>,
}

#[derive(Debug)]
pub struct Component {
	pub name: String,
	pub root: Element,
	pub props: Vec<PropDecl>,
}

#[derive(Debug, Clone)]
pub struct CheckedExpr {
	pub expr: Expr,
	pub expr_type: Type,
	pub bindings: Vec<String>,
}

impl CheckedExpr {
	fn primitive(expr: Expr, expr_type: Type) -> Self {
		CheckedExpr {
			expr,
			expr_type,
			bindings: Vec::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct CheckedRepeater {
	pub collection: CheckedExpr,
	pub index: Option<String>,
	pub item: Option<String>,
	pub item_type: Type,
	pub root_type: String,
}

pub fn check_component(scope: &mut Module, unchecked: &ParserComponent) -> Result<Component, ()> {
	let checked = Component {
		name: unchecked.name.clone(),
		root: check_element(scope, &unchecked.root)?,
		props: unchecked.props.values().cloned().collect(),
	};

	return Ok(checked);
}

fn try_coerce(value: CheckedExpr, t: &Type) -> CheckedExpr {
	if value.expr_type == *t {
		return value;
	}

	match (t, value) {
		(Type::Float, CheckedExpr {
				expr:
					Expr {
						value: ExprValue::Int(n),
						span,
					},
				bindings,
				..
		}) => CheckedExpr {
			expr: Expr {
				value: ExprValue::Float(n as f64),
				span,
			},
			expr_type: Type::Float,
			bindings,
		},
		(Type::String, CheckedExpr { expr, bindings, .. }) => {
			let span = expr.span.clone();
			CheckedExpr {
				expr: Expr {
					value: ExprValue::Coerce(Box::new(expr), Type::String),
					span,
				},
				expr_type: Type::String,
				bindings,
			}
		}
		(_, value) => value,
	}
}

fn check_expr(
	scope: &Module,
	expr: &Expr,
	implicit_type: Option<&Type>,
) -> Result<CheckedExpr, ()> {
	let expr = expr.clone();
	let checked = match expr.value {
		ExprValue::Px(..) => CheckedExpr::primitive(expr, Type::Length),
		ExprValue::Float(..) => CheckedExpr::primitive(expr, Type::Float),
		ExprValue::Int(..) => CheckedExpr::primitive(expr, Type::Int),
		ExprValue::Color(..) => CheckedExpr::primitive(expr, Type::Brush),
		ExprValue::String(..) => CheckedExpr::primitive(expr, Type::String),
		ExprValue::Boolean(..) => CheckedExpr::primitive(expr, Type::Boolean),
		ExprValue::Enum(name, _) => {
			let implicit_type = match implicit_type {
				Some(m) => m,
				None => {
					let message = format!("enum expression is illegal in this context");
					eprintln!("{}", Issue::error(message, expr.span.clone()));
					return Err(());
				}
			};
			let (enum_name, enum_values) = match implicit_type {
				Type::EnumLayout => (
					"Layout".to_owned(),
					hashset!("row".to_owned(), "column".to_owned()),
				),
				_ => {
					let message = format!(
						"expected type `{}`, found unknown enum type",
						implicit_type.name().cyan(),
					);
					eprintln!("{}", Issue::error(message, expr.span.clone()));
					return Err(());
				}
			};
			if !enum_values.contains(&name) {
				let message = format!(
					"`{}` is not a valid member of enum type `{}`",
					name,
					enum_name.cyan(),
				);
				eprintln!("{}", Issue::error(message, expr.span.clone()));
				return Err(());
			}
			CheckedExpr {
				expr: Expr {
					value: ExprValue::Enum(name, Some(enum_name)),
					span: expr.span,
				},
				expr_type: implicit_type.clone(),
				bindings: Vec::new(),
			}
		},
		ExprValue::Object(..) => unimplemented!(),
		ExprValue::Coerce(..) => unimplemented!(),
		ExprValue::FunctionCall(ref fn_expr, ref received_args) => {
			let checked_fn_expr = check_expr(scope, &fn_expr, implicit_type)?;
			let mut bindings = checked_fn_expr.bindings.clone();
			match checked_fn_expr.expr_type {
				Type::Function(expected_args, ret) => {
					if received_args.len() != expected_args.len() {
						let message = format!(
							"expected {} arguments, received {}",
							expected_args.len(),
							received_args.len(),
						);
						eprintln!("{}", Issue::error(message, expr.span.clone()));
						return Err(());
					}
					let mut checked_args = Vec::new();
					for (arg, t) in received_args.iter().zip(expected_args) {
						let checked_arg = type_check(scope, arg, &t)?;
						bindings.append(&mut checked_arg.bindings.clone());
						checked_args.push(checked_arg.expr);
					}
					CheckedExpr {
						expr: Expr {
							value: ExprValue::FunctionCall(
								Box::new(checked_fn_expr.expr),
								checked_args,
							),
							span: expr.span,
						},
						expr_type: *ret,
						bindings,
					}
				}
				_ => {
					let message = format!(
						"expression of type `{}` is not callable",
						checked_fn_expr.expr_type.name().cyan()
					);
					eprintln!(
						"{}",
						Issue::error(message, checked_fn_expr.expr.span.clone())
					);
					return Err(());
				}
			}
		}
		ExprValue::Path(path, ctx) => {
			assert!(ctx == Ctx::Component);
			if let Some((checked_ctx, expr_type)) = scope.lookup(&path, &expr.span)? {
				let bindings = if checked_ctx == Ctx::Component {
					path.clone()
				} else {
					Vec::new()
				};
				CheckedExpr {
					expr: Expr {
						value: ExprValue::Path(path, checked_ctx),
						span: expr.span,
					},
					expr_type,
					bindings,
				}
			} else {
				eprintln!(
					"{}",
					Issue::error(
						format!("property not found: `{}`", path[0]),
						expr.span.clone()
					)
				);
				return Err(());
			}
		}
	};
	if let Some(t) = implicit_type {
		Ok(try_coerce(checked, t))
	} else {
		Ok(checked)
	}
}

fn type_check(scope: &Module, expr: &Expr, t: &Type) -> Result<CheckedExpr, ()> {
	let expr = check_expr(scope, expr, Some(t))?;
	if expr.expr_type != *t {
		let message = format!(
			"expected type `{}`, found `{}`",
			t.name().cyan(),
			expr.expr_type.name().cyan()
		);
		eprintln!("{}", Issue::error(message, expr.expr.span.clone()));
		Err(())
	} else {
		Ok(expr)
	}
}

fn check_child_rules(
	child: &Element,
	def: &ComponentDef,
	parent_span: &Span,
	child_span: &Span,
	rules: ChildRules,
) -> Result<ChildRules, ()> {
	match &rules {
		ChildRules::Any => Ok(rules),
		ChildRules::AnyOf(v) => {
			// TODO: this is hacky and broken. Don't use name to identify type. Same goes for Component & Function Types
			if !v.contains(child.tag.path.last().unwrap()) {
				let permitted = v
					.iter()
					.map(|e| format!("`{e}`"))
					.collect::<Vec<_>>()
					.join(", ");
				let message = format!(
					"invalid child element for `{}` (permitted elements: {permitted})",
					def.name
				);
				eprintln!("{}", Issue::error(message, child_span.clone()));
				return Err(());
			}
			Ok(rules)
		}
		ChildRules::Exact(_) => unimplemented!(),
		ChildRules::ExactCount(_) => unimplemented!(),
		ChildRules::None => {
			let message = format!("`{}` component cannot contain children", def.name);
			eprintln!("{}", Issue::error(message, parent_span.clone()));
			return Err(());
		}
	}
}

pub fn check_element(scope: &mut Module, unchecked: &ParserElement) -> Result<Element, ()> {
	let mut children = Vec::new();

	scope.push_scope();

	assert!(unchecked.condition.is_none() || unchecked.repeater.is_none());

	let condition = if let Some(condition) = &unchecked.condition {
		Some(type_check(scope, &condition.expr, &Type::Boolean)?)
	} else {
		None
	};

	let repeater = if let Some(repeater) = &unchecked.repeater {
		let collection = check_expr(scope, &repeater.collection, None)?;
		let item_type = if let Some(item_type) = collection.expr_type.iter_type() {
			item_type
		} else {
			let message = format!(
				"type `{}` is not iterable",
				collection.expr_type.name().cyan()
			);
			eprintln!("{}", Issue::error(message, collection.expr.span.clone()));
			return Err(());
		};
		if let Some((binding, span)) = &repeater.index {
			scope.declare(binding, &Type::Int, span)?;
		}
		if let Some((binding, span)) = &repeater.item {
			scope.declare(binding, &item_type, span)?;
		}
		assert!(unchecked.path.len() == 1);
		Some(CheckedRepeater {
			index: repeater.index.clone().map(|(s, _)| s),
			item: repeater.item.clone().map(|(s, _)| s),
			collection,
			item_type,
			root_type: unchecked.path[0].clone(),
		})
	} else {
		None
	};

	let component_def = scope.get_component_def(&unchecked)?;
	let mut checked_props = HashMap::new();
	let mut checked_presets = HashMap::new();
	let mut clobbered = Vec::new();
	let mut spans = HashMap::new();
	for (k, p) in unchecked.props.iter() {
		let prop_def = component_def.props.get(k).ok_or_else(|| {
			let message = format!("{k}: no such property");
			eprintln!("{}", Issue::error(message, p.span.clone()));
		})?;
		let expr = type_check(scope, &p.expr, &prop_def.prop_type)?;
		if prop_def.children.len() > 0 {
			spans.insert(k.clone(), p.span.clone());
			checked_presets.insert(k.clone(), expr);
			clobbered.extend(
				prop_def
					.children
					.clone()
					.into_iter()
					.map(|e| (k.clone(), e)),
			);
		} else {
			spans.insert(k.clone(), p.span.clone());
			checked_props.insert(k.clone(), expr);
		}
	}

	for (clobberer, clobbered) in clobbered {
		if checked_props.contains_key(&clobbered) || checked_presets.contains_key(&clobbered) {
			let message = format!("`{clobbered}` is overridden by property `{clobberer}`");
			eprintln!(
				"{}",
				Issue::error(message, spans.get(&clobbered).unwrap().clone())
			);
			return Err(());
		}
	}

	let mut rules = component_def.child_rules.clone();
	for child in unchecked.children.iter() {
		match child {
			ParserContent::Children(c) => {
				children.push(Content::Children(c.clone()));
			}
			ParserContent::Element(e) => {
				let child_span = &e.name_span;
				let e = check_element(scope, &e)?;
				rules = check_child_rules(&e, &component_def, &unchecked.name_span, child_span, rules)?;
				children.push(Content::Element(e));
			}
		}
	}

	let checked = Element {
		tag: ElementTag {
			path: unchecked.path.clone(),
			import_path: None,
		},
		data: None,
		condition,
		repeater,
		props: checked_props,
		presets: checked_presets,
		children: children,
	};

	scope.pop_scope();

	return Ok(checked);
}
