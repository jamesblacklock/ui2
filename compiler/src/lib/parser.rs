use std::collections::HashMap;
use std::cell::Cell;
use std::path::PathBuf;
use convert_case::{Casing, Case};
use maplit::hashmap;

use crate::issue::{Issue};
use crate::source_file::{Span, SourceFile};
use crate::{tokens::*, Expr, Ctx};

use super::{
	Import,
	ExprValue,
	Type,
	PropDecl,
};

#[derive(Debug, Clone)]
pub struct Repeater {
	pub index: Option<(String, Span)>,
	pub item: Option<(String, Span)>,
	pub collection: Expr,
	pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Condition {
	pub expr: Expr,
	pub span: Span,
}

#[derive(Debug)]
pub struct PropAsgn {
	pub expr: Expr,
	pub span: Span,
}

#[derive(Debug)]
pub struct Element {
	pub path: Vec<String>,
	pub data: Option<Expr>,
	pub condition: Option<Condition>,
	pub repeater: Option<Repeater>,
	pub props: HashMap<String, PropAsgn>,
	pub children: Vec<Content>,
	pub name_span: Span,
}

impl Element {
	fn text(value: Expr) -> Self {
		let name_span = value.span.clone();
		Element {
			path: vec!["Text".to_owned()],
			data: None,
			condition: None,
			repeater: None,
			props: hashmap!["content".to_owned() => PropAsgn { expr: value, span: name_span.clone() } ],
			children: vec![],
			name_span,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum CompileStatus {
	Ready,
	Building,
	Done,
}

#[derive(Debug)]
pub struct Component {
	pub name: String,
	pub root: Element,
	pub props: HashMap<String, PropDecl>,
	pub import_decls: Vec<Import>,
	pub imports_map: HashMap<String, std::path::PathBuf>,
	pub status: Cell<CompileStatus>,
}

#[derive(Debug)]
struct Property {
	path: Vec<String>,
	value: Expr,
	span: Span,
}

#[derive(Debug, Clone)]
pub struct Children {
	pub single: bool,
	pub filter: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum Content {
	Element(Element),
	Children(Children),
}

fn hex_to_int(hex: u8) -> u8 {
	if hex >= '0' as u8 && hex <= '9' as u8 {
		hex - '0' as u8
	} else if hex >= 'A' as u8 && hex <= 'F' as u8 {
		10 + (hex - 'A' as u8)
	} else if hex >= 'a' as u8 && hex <= 'f' as u8 {
		10 + (hex - 'a' as u8)
	} else {
		assert!(false, "invalid hex char: {}", hex as char);
		unreachable!();
	}
}

fn color_from_hex(hex: &str) -> ExprValue {
	let hex = hex.as_bytes();
	assert!(hex.len() == 3 || hex.len() == 6);
	match hex.len() {
		3 => {
			let mut r = hex_to_int(hex[0]);
			let mut g = hex_to_int(hex[1]);
			let mut b = hex_to_int(hex[2]);
			r = (r << 4) + r;
			g = (g << 4) + g;
			b = (b << 4) + b;
			ExprValue::Color(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, 1.0)
		},
		4 => {
			let mut r = hex_to_int(hex[0]);
			let mut g = hex_to_int(hex[1]);
			let mut b = hex_to_int(hex[2]);
			let mut a = hex_to_int(hex[3]);
			r = (r << 4) + r;
			g = (g << 4) + g;
			b = (b << 4) + b;
			a = (a << 4) + a;
			ExprValue::Color(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
		},
		6 => {
			let r = (hex_to_int(hex[0]) << 4) + hex_to_int(hex[1]);
			let g = (hex_to_int(hex[2]) << 4) + hex_to_int(hex[3]);
			let b = (hex_to_int(hex[4]) << 4) + hex_to_int(hex[5]);
			ExprValue::Color(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, 1.0)
		},
		8 => {
			let r = (hex_to_int(hex[0]) << 4) + hex_to_int(hex[1]);
			let g = (hex_to_int(hex[2]) << 4) + hex_to_int(hex[3]);
			let b = (hex_to_int(hex[4]) << 4) + hex_to_int(hex[5]);
			let a = (hex_to_int(hex[6]) << 4) + hex_to_int(hex[7]);
			ExprValue::Color(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, a as f64 / 255.0)
		},
		_ => unreachable!(),
	}
}

struct Parser {
	file_path: PathBuf,
	tokens: Vec<Token>,
	offset: usize,
	failed: bool,
}

macro_rules! def_token_matchers {
	($permit:ident, $expect:ident, $descr:expr, $tt:ident, $t:ty) => {
		fn $permit(&mut self) -> Option<($t, Span)> {
			if let TT::$tt(data) = &self.cur().tok {
				let data = data.clone();
				let span = self.cur().span.clone();
				self.offset += 1;
				Some((data, span))
			} else {
				None
			}
		}

		fn $expect(&mut self) -> Result<($t, Span), ()> {
			self.$permit().ok_or_else(|| {
				let tok = self.cur().clone();
				self.expected_error($descr, &tok);
			})
		}
	}
}

impl Parser {
	fn new(file_path: &PathBuf, tokens: Vec<Token>) -> Self {
		Parser {
			file_path: file_path.clone(),
			tokens,
			offset: 0,
			failed: false,
		}
	}

	fn error<S: Into<String>>(&mut self, message: S, span: &Span) {
		eprintln!("{}", Issue::error(message, span.clone()));
		self.failed = true;
	}

	fn expected_error<S: Into<String>>(&mut self, expected: S, found: &Token) {
		self.error(format!("expected {}, found {}", expected.into(), found), &found.span)
	}

	fn cur(&self) -> &Token {
		&self.tokens[self.offset]
	}

	fn cur_offset(&self, offset: isize) -> &Token {
		if offset < 0 {
			&self.tokens[self.offset - offset.abs() as usize]
		} else {
			&self.tokens[self.offset + offset as usize]
		}
	}

	fn offset(&self) -> usize {
		return self.offset;
	}

	fn set_offset(&mut self, offset: usize) {
		self.offset = offset;
	}

	fn permit(&mut self, tok: TT) -> Option<Token> {
		if self.cur().tok == tok {
			self.offset += 1;
			Some(self.cur_offset(-1).clone())
		} else {
			None
		}
	}

	fn expect(&mut self, expected: TT) -> Result<Token, ()> {
		let desc = expected.desc();
		self.permit(expected).ok_or_else(|| self.expected_error(desc, &self.cur().clone()))
	}

	def_token_matchers! { permit_string, expect_string, "string", String, String }
	def_token_matchers! { permit_name, expect_name, "name", Name, String }
	def_token_matchers! { permit_number, expect_number, "number", Number, (String, bool, String) }
	def_token_matchers! { permit_hex_color, expect_hex_color, "hex_color", HexColor, String }
	def_token_matchers! { permit_enum, expect_enum, "enum", Enum, String }

	fn expect_statement_separator(&mut self) -> Result<(), ()> {
		if !(self.cur().is(TT::RBrace) || self.cur().is(TT::RParen) || self.cur().is(TT::RBrack)) {
			self.expect(TT::Semicolon)?;
		}
		Ok(())
	}

	fn parse_type(&mut self) -> Result<(Type, Span), ()> {
		if let Some((name, span)) = self.permit_name() {
			match name.as_str() {
				"Int" => Ok((Type::Int, span.clone())),
				"Float" => Ok((Type::Float, span.clone())),
				"Length" => Ok((Type::Length, span.clone())),
				"Brush" => Ok((Type::Brush, span.clone())),
				"String" => Ok((Type::String, span.clone())),
				"Boolean" => Ok((Type::Boolean, span.clone())),
				"Alignment" => Ok((Type::Alignment, span.clone())),
				"Callback" => Ok((Type::Callback, span.clone())),
				_ => {
					self.error(format!("unrecognized type: {}", name), &span);
					Err(())
				}
			}
		} else {
			unimplemented!()
		}
	}
	
	fn parse_imports(&mut self) -> Result<Vec<Import>, ()> {
		let mut imports = Vec::new();

		while let Some(Token { span, .. }) = self.permit(TT::Import) {
			let path = PathBuf::from(self.expect_string()?.0);
			let alias = if self.permit(TT::As).is_some() {
				Some(self.expect_name()?.0)
			} else {
				None
			};
			let end_span = self.expect(TT::Semicolon)?.span;

			imports.push(Import { path, alias, span: span.merge(&end_span) });
		}
		Ok(imports)
	}

	fn parse_prop_decls(&mut self) -> Result<HashMap<String, PropDecl>, ()> {
		let mut props: Vec<PropDecl> = Vec::new();

		loop {
			let (is_pub, name, start_span) = if let Some(pub_token) = self.permit(TT::Pub) {
				(true, self.expect_name()?.0, pub_token.span)
			} else if self.cur().is_name() && self.cur_offset(1).is(TT::Colon) {
				let (name, span) = self.expect_name()?;
				(false, name, span)
			} else {
				break;
			};

			self.expect(TT::Colon)?;

			let (prop_type, type_span) = self.parse_type()?;
			props.push(PropDecl {
				is_pub,
				name: name,
				prop_type,
				default: None,
				span: start_span.merge(&type_span),
			});


			self.expect(TT::Semicolon)?;
		}

		let map = props.into_iter().fold(HashMap::new(), |mut map, e| {
			if map.contains_key(&e.name) {
				self.error(format!("property `{}` declared more than once", e.name), &e.span);
			} else {
				map.insert(e.name.clone(), e);
			}
			map
		});

		return Ok(map);
	}

	fn parse_path(&mut self) -> Result<(Vec<String>, Span), ()> {
		let (name, mut full_span) = self.expect_name()?;
		let mut path = vec![name];
		loop {
			if let Some(_) = self.permit(TT::Period) {
				let (name, span) = self.expect_name()?;
				full_span = full_span.merge(&span);
				path.push(name);
			} else if let Some((name, span)) = self.permit_enum() {
				full_span = full_span.merge(&span);
				path.push(name);
			} else {
				break;
			}
		}

		Ok((path, full_span))
	}

	fn parse_function_call(&mut self, expr: Expr) -> Result<Expr, ()> {
		self.expect(TT::LParen)?;

		let mut args = Vec::new();
		while !(self.cur().is(TT::RParen) || self.cur().is(TT::Eof)) {
			args.push(self.parse_expr()?);
			if !(self.cur().is(TT::Comma) || self.cur().is(TT::RParen)) {
				self.error(format!("expected `,` or `)`, found {}", self.cur()), &self.cur().span.clone());
				return Err(());
			}
			self.permit(TT::Comma);
		}

		let rparen = self.expect(TT::RParen)?;
		let span = expr.span.merge(&rparen.span);
		Ok(Expr {
			value: ExprValue::FunctionCall(Box::new(expr), args),
			span,
		})
	}

	fn parse_expr(&mut self) -> Result<Expr, ()> {
		let paren = self.permit(TT::LParen).is_some();

		let mut expr = if self.cur().is_name() {
			let (path, span) = self.parse_path()?;
			Expr { value: ExprValue::Path(path, Ctx::Component), span }
		} else {
			self.parse_value()?
		};

		loop {
			if self.cur().is(TT::LParen) {
				expr = self.parse_function_call(expr)?;
			} else {
				break;
			}
		}

		if paren {
			self.expect(TT::RParen)?;
		}

		Ok(expr)
	}

	fn parse_value(&mut self) -> Result<Expr, ()> {
		let num_tok = if let Some(tok) = self.permit(TT::Minus) {
			let (num, span) = self.expect_number()?;
			Some((num, true, tok.span.merge(&span)))
		} else if let Some(tok) = self.permit(TT::Plus) {
			let (num, span) = self.expect_number()?;
			Some((num, false, tok.span.merge(&span)))
		} else {
			self.permit_number().map(|(num, span)| (num, false, span))
		};
		if let Some(((num, float, suffix), negative, span)) = num_tok {
			let n = num.parse::<f64>().unwrap() * if negative { -1.0 } else { 1.0 };
			match suffix.as_str() {
				"px" => Ok(Expr { value: ExprValue::Px(n), span }),
				"" => {
					let value = if float { ExprValue::Float(n) } else { ExprValue::Int(n as i64) };
					Ok(Expr { value, span })
				},
				_ => {
					self.error(format!("unrecognized numerical suffix: `{}`", suffix), &span);
					Err(())
				},
			}
		} else if let Some((hex_tok, span)) = self.permit_hex_color() {
			Ok(Expr { value: color_from_hex(&hex_tok), span })
		} else if let Some(Token { span, .. }) = self.permit(TT::True) {
			Ok(Expr { value: ExprValue::Boolean(true), span })
		} else if let Some(Token { span, .. }) = self.permit(TT::False) {
			Ok(Expr { value: ExprValue::Boolean(false), span })
		} else if let Some((s, span)) = self.permit_string() {
			Ok(Expr { value: ExprValue::String(s), span })
		} else if let Some((s, span)) = self.permit_enum() {
			Ok(Expr { value: ExprValue::Enum(s, None), span })
		} else if self.cur().tok == TT::LParen {
			self.parse_expr()
		} else {
			self.expected_error("value expression", &self.cur().clone());
			Err(())
		}
	}

	fn parse_prop_assignments(&mut self) -> Result<HashMap<String, PropAsgn>, ()> {
		let mut props = Vec::new();
		loop {
			let offset = self.offset();

			if !self.cur().is_name() {
				break;
			}

			let (path, mut full_span) = self.parse_path()?;

			let value = if let Some(_) = self.permit(TT::Colon) {
				self.parse_value()?
			} else {
				self.set_offset(offset);
				break
			};

			full_span = full_span.merge(&value.span);

			self.expect_statement_separator()?;

			props.push(Property {
				path,
				value,
				span: full_span,
			});
		}

		fn add_property(map: &mut HashMap<String, PropAsgn>, path: &[String], prop: &Property) -> Result<(), (String, Span)> {
			if path.len() == 1 {
				if map.contains_key(&path[0]) {
					return Err((format!("property `{}` assigned more than once", path[0]), prop.span.clone()));
				} else {
					map.insert(path[0].clone(), PropAsgn { expr: prop.value.clone(), span: prop.span.clone() });
				}
			} else {
				unimplemented!();
			}
			Ok(())
		}

		let mut props_map = HashMap::new();
		for prop in props.into_iter() {
			if let Err((message, span)) = add_property(&mut props_map, &prop.path, &prop) {
				self.error(message, &span);
				return Err(());
			}
		}
		
		Ok(props_map)
	}

	fn parse_element(&mut self) -> Result<Element, ()> {
		let (path, name_span) = self.parse_path()?;

		let illegal_prop_message = "property assignments must occur before any content definitions";

		let repeater = if let Some(Token { span: for_span, .. }) = self.permit(TT::For) {
			let item_or_index = {
				let (item, item_span) =	self.expect_name()?;
				if item == "_" { None } else { Some((item, item_span)) }
			};

			let (index, item) = if self.permit(TT::Comma).is_some() {
				let (item, item_span) =	self.expect_name()?;
				(item_or_index, if item == "_" { None } else { Some((item, item_span)) })
			} else {
				(None, item_or_index)
			};

			self.expect(TT::In)?;
			let collection = self.parse_value()?;
			let span = for_span.merge(&collection.span);
			Some(Repeater {
				index,
				item,
				collection,
				span,
			})
		} else {
			None
		};

		self.expect(TT::LBrace)?;
		let props = self.parse_prop_assignments()?;

		let mut children = Vec::new();
		loop {
			if self.cur().is_name() {
				if self.cur_offset(1).is(TT::Colon) {
					self.error(illegal_prop_message, &self.cur().span.clone());
					return Err(());
				}
				let child = self.parse_element()?;
				children.push(Content::Element(child));
			} else if let Some((value, span)) = self.permit_string() {
				let value = Expr { value: ExprValue::String(value), span };
				children.push(Content::Element(Element::text(value)));
			} else if self.cur().is(TT::LParen) {
				let value = self.parse_expr()?;
				children.push(Content::Element(Element::text(value)));
			} else if self.cur().is(TT::Pub) {
				self.error(illegal_prop_message, &self.cur().span.clone());
				return Err(());
			} else {
				break;
			}
		}
		self.expect(TT::RBrace)?;

		Ok(Element {
			path,
			data: None,
			condition: None,
			repeater,
			props,
			children,
			name_span,
		})
	}

	pub fn parse(&mut self) -> Result<Component, ()> {
		let imports = self.parse_imports()?;
		let props = self.parse_prop_decls()?;
		let root = self.parse_element()?;

		if !self.cur().is(TT::Eof) {
			let span = self.cur().span.clone();
			if self.cur().is_name() && self.cur_offset(1).is(TT::Colon) {
				 self.error("property declarations must occur before any content definitions", &span);
			} else if self.cur_offset(1).is(TT::Pub) {
				 self.error("property declarations must occur before any content definitions", &span);
			} else if self.cur().is_name() && self.cur_offset(1).is(TT::LBrace) {
				self.error("the component must have a single root element", &span);
			} else {
				self.expected_error("end of file", &self.cur().clone());
			}
			return Err(());	
		}

		if let Some(condition) = root.condition.as_ref() {
			self.error("`if ...` cannot be used on the root of the component", &condition.span);
			return Err(());
		} else if let Some(repeater) = root.repeater.as_ref() {
			self.error("`for ... in ...` cannot be used on the root of the component", &repeater.span);
			return Err(());
		}

		let file_stem: String = self.file_path
			.file_stem()
			.unwrap()
			.to_string_lossy()
			.into();
		let name = file_stem.to_case(Case::UpperCamel);

		if self.failed {
			Err(())
		} else {
			Ok(Component {
				name,
				props,
				root,
				import_decls: imports,
				imports_map: HashMap::new(),
				status: Cell::new(CompileStatus::Ready),
			})
		}
	}
}

pub fn parse(file_path: &PathBuf) -> Result<Component, ()> {
	let source_file = SourceFile::load(file_path)?;
	let tokens = Tokenizer::new(source_file).tokenize()?;
	Parser::new(file_path, tokens).parse()
}

