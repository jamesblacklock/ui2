use ui_compiler::*;
use std::process;

#[derive(Default)]
struct Options {
	file: String,
	watch: bool,
}

fn process_args() -> Options {
	let mut args = std::env::args();
	let exe = args.next().unwrap();
	let mut file = None;
	let mut watch = None;
	let mut fail = false;

	for arg in args {
		match arg.as_str() {
			"--watch" =>  {
				if watch.is_some() {
					fail = true;
				}
				watch = Some(true);
			},
			_ => {
				if file.is_some() {
					fail = true;
				}
				file = Some(arg);
			}
		}
	}

	if fail || file.is_none() {
		eprintln!("usage: {} FILE [--watch]", exe);
		process::exit(1);
	}

	Options {
		file: file.unwrap(),
		watch: watch.unwrap_or_default(),
	}
}

fn main() {
	let options = process_args();
	if options.watch {
		watch(&options.file);
	} else if let Err(_) = build(&options.file) {
		process::exit(1);
	}
}
