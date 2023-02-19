use std::{fmt};
use colored::*;

use crate::source_file::Span;

#[derive(PartialEq)]
pub enum IssueLevel {
  Warning,
  Error,
}

pub struct Issue {
  pub level: IssueLevel,
  pub span: Span,
  pub message: String,
}

impl Issue {
  pub fn warning<S: Into<String>>(message: S, span: Span) -> Self {
    Issue {
      level: IssueLevel::Warning,
      message: message.into(),
      span,
    }
  }
  pub fn error<S: Into<String>>(message: S, span: Span) -> Self {
    Issue {
      level: IssueLevel::Error,
      message: message.into(),
      span,
    }
  }
}

impl fmt::Display for Issue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let notice = match self.level {
      IssueLevel::Warning => "warning".yellow().bold(),
      IssueLevel::Error => "error".red().bold(),
    };

    let lines_start = if self.span.start_line < 3 {
      1
    } else {
      self.span.start_line - 2
    };
    let lines_end = if self.span.end_line + 2 >= self.span.source_file.lines.len() {
      self.span.source_file.lines.len() - 1
    } else {
      self.span.end_line + 2
    };

    let gutter_width = (lines_end as f64).log10().floor() as usize + 4;

    write!(f, "{}: {}\n{:>gutter_width$} {}:{}:{}",
      notice,
      self.message.bold(),
      " -->".blue().bold(),
      self.span.source_file.file_path.display(),
      self.span.start_line,
      self.span.start_column,
    )?;

    for i in lines_start..self.span.start_line {
      let gutter = format!("{} | ", i).blue().bold();
      write!(f, "\n{:>gutter_width$}{}", gutter, self.span.source_file.lines[i])?;
    }

    for i in self.span.start_line..=self.span.end_line {
      let gutter = format!("{} | ", i).blue().bold();

      let line = &self.span.source_file.lines[i];

      let start = if i == self.span.start_line { self.span.start_column-1 } else { 0 };
      let end = if i == self.span.end_line { self.span.end_column-1 } else { line.len() };

      let before = line[0..start].to_owned();
      let highlight = line[start..end].to_owned();
      let after = line[end..].to_owned();

      let highlight = if self.level == IssueLevel::Error { highlight.red() } else { highlight.yellow() };

      write!(f, "\n{:>gutter_width$}{}{}{}", gutter, before, highlight.red().bold(), after)?;
    }

    for i in (self.span.end_line+1)..=lines_end {
      let gutter = format!("{} | ", i).blue().bold();
      write!(f, "\n{:>gutter_width$}{}", gutter, self.span.source_file.lines[i])?;
    }

    Ok(())
  }
}
