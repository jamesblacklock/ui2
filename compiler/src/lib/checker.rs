use std::path::PathBuf;
use std::{fmt::Debug};
use std::collections::HashMap;
use colored::*;

use crate::source_file::Span;
use crate::{ValueExpr, Expr};
use crate::issue::{Issue};
use crate::parser::Condition;

use super::{
	parser::Element as ParserElement,
	parser::Content as ParserContent,
	parser::Component as ParserComponent,
	parser::Repeater,
	Module,
	Value,
	Type,
	PropDecl,
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
	pub data: Option<Value>,
	pub condition: Option<Condition>,
	pub repeater: Option<Repeater>,
	pub props: HashMap<String, ValueExpr>,
	pub children: Vec<Content>,
}

#[derive(Debug)]
pub struct Component {
	pub name: String,
	pub root: Element,
	pub props: Vec<PropDecl>,
}

pub fn check_component(scope: &mut Module, unchecked: &ParserComponent) -> Result<Component, ()> {
	let checked = Component {
		name: unchecked.name.clone(),
		root: check_element(scope, &unchecked.root)?,
		props: unchecked.props.values().cloned().collect(),
	};

	return Ok(checked);
}

fn check_expr(scope: &Module, expr: &Expr, span: &Span) -> Result<Type, ()> {
	match expr {
		Expr::Path(path, ctx) => {
			assert!(path.len() == 1);
			if let Some(p) = scope.props.get(&path[0]) {
				Ok(p.prop_type.clone())
			} else {
				eprintln!("{}", Issue::error(format!("property not found: `{}`", path[0]), span.clone()));
				Err(())
			}
		}
	}
}

pub fn type_check(scope: &Module, expected_type: &Type, expr: &ValueExpr) -> Result<(), ()> {
	let found_type = match &expr.value {
		Value::Px(..) => Type::Length,
		Value::Color(..) => Type::Brush,
		Value::String(..) => Type::String,
		Value::Boolean(..) => Type::Boolean,
		Value::Expr(e) => check_expr(scope, &e, &expr.span)?,
		_ => unimplemented!(),
	};

	if found_type != *expected_type {
		let message = format!("expected type `{}`, found `{}`", expected_type.name().cyan(), found_type.name().cyan());
		eprintln!("{}", Issue::error(message, expr.span.clone()));
		Err(())
	} else {
		Ok(())
	}
}

pub fn check_element(scope: &mut Module, unchecked: &ParserElement) -> Result<Element, ()> {
	let mut children = Vec::new();

	let component_def = scope.get_component_def(&unchecked)?;
	for (k, p) in unchecked.props.iter() {
		let prop_type = component_def.props.get(k).ok_or_else(|| {
			let message = format!("{k}: no such property");
			eprintln!("{}", Issue::error(message, p.span.clone()));
		})?;
		type_check(scope, prop_type, p)?;
	}

	for child in unchecked.children.iter() {
		match child {
			ParserContent::Children(c) => {
				children.push(Content::Children(c.clone()));
			},
			ParserContent::Element(e) => {
				children.push(Content::Element(check_element(scope, &e)?));
			},
		}
	}

	let checked = Element {
		tag: ElementTag { path: unchecked.path.clone(), import_path: None },
		data: unchecked.data.clone(),
		condition: unchecked.condition.clone(),
		repeater: unchecked.repeater.clone(),
		props: unchecked.props.clone(),
		children: children,
	};

	return Ok(checked);
}
