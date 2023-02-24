use std::{path::PathBuf, rc::Rc, fs, io::Read, iter};

#[derive(Debug)]
pub struct SourceFile {
  pub file_path: PathBuf,
  pub buf: String,
  pub lines: Vec<String>,
  pub internal: bool,
}

impl SourceFile {
  pub fn internal() -> Rc<Self> {
    Rc::new(SourceFile {
      file_path: "<internal>".into(),
      buf: "".into(),
      lines: Vec::new(),
      internal: true,
    })
  }

  pub fn load(file_path: &PathBuf) -> Result<Rc<Self>, ()> {
    let mut buf = String::new();
		fs::File::open(&file_path)
      .or_else(|_| {
        eprintln!("could not load file: {}", file_path.display());
        Err(())
      })?
      .read_to_string(&mut buf)
      .unwrap();
    let mut lines: Vec<_> = iter::once("").chain(buf.lines()).map(|s| s.to_owned()).collect();
    if lines.last() != Some(&String::new()) {
      lines.push(String::new());
    }

    Ok(Rc::new(SourceFile {
      file_path: file_path.clone(),
      buf,
      lines,
      internal: false,
    }))
  }
}

impl PartialEq for SourceFile {
  fn eq(&self, other: &Self) -> bool {
    self.file_path == other.file_path
  }
}

#[derive(Debug, Clone)]
pub struct Span {
  pub start_line: usize,
  pub start_column: usize,
  pub end_line: usize,
  pub end_column: usize,
  pub source_file: Rc<SourceFile>,
}

impl Span {
  pub fn internal() -> Span {
    Span {
      start_line: 0,
      start_column: 0,
      end_line: 0,
      end_column: 0,
      source_file: SourceFile::internal(),
    }
  }

  pub fn merge(&self, other: &Span) -> Span {
    assert!(self.source_file == other.source_file);
    assert!(self.start_line <= other.start_line);
    let start_line = std::cmp::min(self.start_line, other.start_line);
    let end_line = std::cmp::max(self.end_line, other.end_line);
    if start_line == end_line {
      Span {
        start_line,
        end_line,
        start_column: std::cmp::min(self.start_column, other.start_column),
        end_column: std::cmp::max(self.end_column, other.end_column),
        source_file: self.source_file.clone(),
      }
    } else {
      Span {
        start_line,
        end_line,
        start_column: self.start_column,
        end_column: other.end_column,
        source_file: self.source_file.clone(),
      }
    }
  }
}
