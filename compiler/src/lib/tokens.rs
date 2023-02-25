use std::{fmt, rc::Rc, mem};

use crate::{issue::Issue, source_file::{SourceFile, Span}};

pub struct Tokenizer<'a> {
	source_file: Rc<SourceFile>,
	input: &'a str,
	line: usize,
	column: usize,
	failed: bool,
	eof: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TT {
	Name(String),
	Enum(String),
	String(String),
	Number((String, bool, String)),
	HexColor(String),
	True,
	False,
	Pub,
	Import,
	As,
	For,
	In,
	If,
	LBrace,
	RBrace,
	LParen,
	RParen,
	Colon,
	Semicolon,
	Plus,
	Minus,
	Asterisk,
	Period,
	Comma,
	Slash,
	Err(String),
	Eof
}

impl TT {
	pub fn desc(&self) -> String {
		match self {
			TT::Name(..) => "name".to_owned(),
			TT::Enum(..) => "enum".to_owned(),
			TT::String(..) => "string".to_owned(),
			TT::Number(..) => "number".to_owned(),
			TT::HexColor(..) => "hex color".to_owned(),
			TT::True => "true".to_owned(),
			TT::False => "false".to_owned(),
			TT::Pub => "pub".to_owned(),
			TT::Import => "import".to_owned(),
			TT::As => "as".to_owned(),
			TT::For => "for".to_owned(),
			TT::In => "in".to_owned(),
			TT::If => "if".to_owned(),
			TT::LBrace => "{".to_owned(),
			TT::RBrace => "}".to_owned(),
			TT::LParen => "(".to_owned(),
			TT::RParen => ")".to_owned(),
			TT::Colon => ":".to_owned(),
			TT::Semicolon => ";".to_owned(),
			TT::Plus => "+".to_owned(),
			TT::Minus => "-".to_owned(),
			TT::Asterisk => "*".to_owned(),
			TT::Period => ".".to_owned(),
			TT::Comma => ",".to_owned(),
			TT::Slash => "/".to_owned(),
			TT::Eof => "end of file".to_owned(),
			TT::Err(_) => unreachable!(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Token {
	pub tok: TT,
	pub span: Span,
}

impl Token {
	pub fn is(&self, tok: TT) -> bool {
		self.tok == tok
	}
	pub fn is_name(&self) -> bool {
		match self.tok {
			TT::Name(_) => true,
			_ => false,
		}
	}
	pub fn is_string(&self) -> bool {
		match self.tok {
			TT::String(_) => true,
			_ => false,
		}
	}
	pub fn is_enum(&self) -> bool {
		match self.tok {
			TT::Enum(_) => true,
			_ => false,
		}
	}
}

fn string_repr(unescaped: &String) -> String {
	let s = unescaped
		.replace("\n", "\\n")
		.replace("\t", "\\t")
		.replace("\"", "\\\"");
	format!("\"{}\"", s)
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match &self.tok {
			TT::Name(s) => s.clone(),
			TT::String(s) => string_repr(s),
			TT::Number((s1, _, s2)) => format!("{s1}{s2}"),
			TT::HexColor(s) => format!("#{s}"),
			TT::Err(s) => string_repr(s),
			_ => self.tok.desc(),
		};
		write!(f, "`{}`", s)
	}
}

fn is_space(c: char) -> bool {
	c == ' ' || c == '\n' || c == '\t'
}

fn is_name(c: char) -> bool {
	is_name_first(c) ||	is_digit(c)
}

fn is_digit(c: char) -> bool {
	c >= '0' && c <= '9'
}

fn is_hex_digit(c: char) -> bool {
	is_digit(c) ||
	c >= 'a' && c <= 'f' ||
	c >= 'A' && c <= 'F'
}

fn is_name_first(c: char) -> bool {
	c >= 'A' && c <= 'Z' ||
	c >= 'a' && c <= 'z' ||
	c == '_'
}

fn is_op_one(c: char) -> bool {
	c == '{' || c == '}' || c == '(' || c == ')' ||
	c == ':' || c == ';' || c == '+' || c == '-' ||
	c == '*' || c == ','
}

impl <'a> Tokenizer<'a> {
	pub fn new(source_file: Rc<SourceFile>) -> Self {
		let buf = source_file.as_ref().buf.as_str() as *const str;
		Self {
			source_file,
			input: unsafe { mem::transmute(buf) },
			line: 1,
			column: 1,
			failed: false,
			eof: false,
		}
	}

	pub fn tokenize(mut self) -> Result<Vec<Token>, ()> {
		let mut tokens = Vec::new();
		let mut eof = false;
		while !eof {
			let t = self.next().unwrap();
			eof = t.tok == TT::Eof;
			tokens.push(t);
		}
		if self.failed {
			Err(())
		} else {
			Ok(tokens)
		}
	}

	fn error<S: Into<String>>(&mut self, message: S, span: Span) {
		eprintln!("{}", Issue::error(message.into(), span));
		self.failed = true;
	}

	fn consume(&mut self, f: fn(char) -> bool) -> (String, Span) {
		let mut it = self.input.char_indices();
		let start_line = self.line;
		let start_column = self.column;
		while let Some((i, c)) = it.next() {
			if !f(c) {
				let s = self.input[0..i].to_owned();
				self.input = &self.input[i..];
				return (s, Span {
					start_line,
					start_column,
					end_line: self.line,
					end_column: self.column,
					source_file: self.source_file.clone(),
				});
			}
			if c == '\n' {
				self.line += 1;
				self.column = 1;
			} else {
				self.column += 1;
			}
		}

		let s = self.input.to_owned();
		self.input = &self.input[0..0];
		(s, Span {
			start_line,
			start_column,
			end_line: self.line,
			end_column: self.column,
			source_file: self.source_file.clone(),
		})
	}

	fn consume_single_char(&mut self) -> (char, Span) {
		let start_line = self.line;
		let start_column = self.column;
		let mut iter = self.input.char_indices();
		let (_, c) = iter.next().unwrap();
		if c == '\n' {
			self.line += 1;
			self.column = 1;
		} else {
			self.column += 1;
		}
		if let Some((i, _)) = iter.next() {
			self.input = &self.input[i..];
		} else {
			self.input = &self.input[0..0];
		}

		(c, Span {
			start_line,
			start_column,
			end_line: self.line,
			end_column: self.column,
			source_file: self.source_file.clone(),
		})
	}

	fn skip_space(&mut self) {
		self.consume(is_space);
	}

	fn skip_multiline_comment(&mut self) {
		let mut it = self.input.char_indices().skip(2);
		self.column += 2;

		let mut depth = 1;
		let mut offset = 0;
		while depth > 0 {
			if let Some((n1, c1)) = it.next() {
				offset = n1 + 1;
				if c1 == '\n' {
					self.line += 1;
					self.column = 1;
				} else {
					self.column += 1;
				}
				if c1 == '/' || c1 == '*' {
					if let Some((n2, c2)) = it.next() {
						offset = n2 + 1;
						if c1 == '\n' {
							self.line += 1;
							self.column = 1;
						} else {
							self.column += 1;
						}
						if c1 == '/' && c2 == '*' {
							depth += 1;
						} else if c1 == '*' && c2 == '/' {
							depth -= 1;
						}
					}
				}
			} else {
				break;
			}
		}

		self.input = &self.input[offset..];
	}

	fn op_one_token(&mut self, c: char, span: Span) -> Token {
		match c {
			'{' => Token { tok: TT::LBrace, span },
			'}' => Token { tok: TT::RBrace, span },
			'(' => Token { tok: TT::LParen, span },
			')' => Token { tok: TT::RParen, span },
			':' => Token { tok: TT::Colon, span },
			';' => Token { tok: TT::Semicolon, span },
			'+' => Token { tok: TT::Plus, span },
			'-' => Token { tok: TT::Minus, span },
			'*' => Token { tok: TT::Asterisk, span },
			',' => Token { tok: TT::Comma, span },
			_ => unreachable!()
		}
	}

	fn name_token(&mut self, data: String, span: Span) -> Token {
		match data.as_str() {
			"true" => Token { tok: TT::True, span },
			"false" => Token { tok: TT::False, span },
			"pub" => Token { tok: TT::Pub, span },
			"import" => Token { tok: TT::Import, span },
			"as" => Token { tok: TT::As, span },
			"for" => Token { tok: TT::For, span },
			"in" => Token { tok: TT::In, span },
			"if" => Token { tok: TT::If, span },
			_ => Token { tok: TT::Name(data), span }
		}
	}

	fn hex_color_token(&mut self, data: String, span: Span) -> Token {
		match data.len() {
			3|4|6|8 => Token { tok: TT::HexColor(data), span },
			_ => {
				self.error(format!("expected 3, 4, 6, or 8 hex digits (found {})", data.len()), span.clone());
				Token { tok: TT::Err(data), span }
			},
		}
	}

	fn number_token(&mut self, num: String, float: bool, suffix: String, span: Span) -> Token {
		Token { tok: TT::Number((num, float, suffix)), span }
	}
}

impl <'a> Iterator for Tokenizer<'a> {
	type Item = Token;
	fn next(&mut self) -> Option<Token> {
		self.skip_space();
		while self.input.len() > 0 {
			let c = self.input.chars().nth(0).unwrap();
			let c2 = self.input.chars().nth(1);
			if is_name_first(c) {
				let (data, span) = self.consume(is_name);
				return Some(self.name_token(data, span));
			} else if is_digit(c) {
				let (mut num, mut span) = self.consume(is_digit);
				let maybe_dot = self.input.chars().nth(0);
				let float = if maybe_dot == Some('.') {
					let (_, dot_span) = self.consume_single_char();
					let (num2, span2) = self.consume(is_digit);
					if num2 == "" {
						let span = span.merge(&dot_span);
						self.error("expected digits following decimal point".to_owned(), span.clone());
						return Some(Token { tok: TT::Err(num + "."), span });
					}
					num = format!("{num}.{num2}");
					span = span.merge(&span2);
					true
				} else {
					false
				};
				let (suffix, suffix_span) = self.consume(is_name);
				return Some(self.number_token(num, float, suffix, span.merge(&suffix_span)));
			} else if c == '#' {
				let (_, hash_span) = self.consume_single_char();
				let (hex, hex_span) = self.consume(is_hex_digit);
				return Some(self.hex_color_token(hex, hash_span.merge(&hex_span)));
			} else if c == '"' {
				let (_, quote1_span) = self.consume_single_char();
				let (content, content_span) = self.consume(|c| c != '"');
				if self.input.len() == 0 {
					let span = quote1_span.merge(&content_span);
					self.error("encountered unterminated string literal".to_owned(), span.clone());
					return Some(Token { tok: TT::Err(content), span });
				}
				let (_, quote2_span) = self.consume_single_char();
				return Some(Token { tok: TT::String(content), span: quote1_span.merge(&quote2_span) });
			} else if c == '/' {
				if c2 == Some('/') {
					self.consume(|c| c != '\n');
				} else if c2 == Some('*') {
					self.skip_multiline_comment();
				} else {
					let (_, span) = self.consume_single_char();
					return Some(Token { tok: TT::Slash, span });
				}
			} else if c == '.' && is_name_first(c2.unwrap_or('\u{00}')) {
				let (_, dot_span) = self.consume_single_char();
				let (name, name_span) = self.consume(is_name);
				return Some(Token { tok: TT::Enum(name), span: dot_span.merge(&name_span) });
			} else if is_op_one(c) {
				let (c, span) = self.consume_single_char();
				return Some(self.op_one_token(c, span));
			} else {
				let (c, span) = self.consume_single_char();
				self.error(format!("encountered illegal character: '{} ({})'", c, c.escape_unicode()), span.clone());
				return Some(Token { tok: TT::Err(c.to_string()), span });
			}

			self.skip_space();
		}

		if !self.eof {
			self.eof = true;
			Some(Token {
				tok: TT::Eof,
				span: Span {
					start_line: self.line,
					start_column: self.column,
					end_line: self.line,
					end_column: self.column,
					source_file: self.source_file.clone(),
				}
			})
		} else {
			None
		}
	}
}
