extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use ansi_term::Colour;
use docopt::Docopt;

use std::fs;
use std::path;

const USAGE: &'static str = "
srch, a command-line file search utility written in Rust.

Usage:
    srch [options] <pattern>
    srch --help
    srch --version

Options:
    -h, --help      Show this help screen
    -v, --version   Show this program's version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_pattern: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).unwrap()
                            .help(true)
                            .version(Some(String::from("srch, version ") + option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")))
                            .decode()
                            .unwrap_or_else(|e| e.exit());

    let curdir = path::PathBuf::from("./");
    explore(&curdir, &args.arg_pattern);
}

fn ignore (path: &path::PathBuf) -> bool {
    let fname = path.file_name();
    if fname == None {
        return false
    }

    let fname = fname.unwrap().to_str().unwrap();
    if fname.chars().next().unwrap() == '.' {
        return true
    }

    false
}

trait Misc {
    fn is_dir(&self) -> bool;
    fn display_colour(&self) -> String;
    fn matches(&self, pattern: &String) -> bool;
}

impl Misc for path::PathBuf {
    fn is_dir(&self) -> bool {
        fs::metadata(self).unwrap().is_dir()
    }

    fn display_colour(&self) -> String {
        let s: &str = self.to_str().unwrap();

        if self.is_dir() {
            Colour::Yellow.bold().paint(s).to_string()
        } else {
            s.to_string()
        }
    }

    fn matches(&self, pattern: &String) -> bool {
        let fname = self.file_name();
        if fname == None {
            return false
        }

        let fname = fname.unwrap().to_str().unwrap().to_string();
        if fname == *pattern {
            return true
        }
        false
    }
}

fn explore(path: &path::PathBuf, pattern: &String) -> () {
    if !path.is_dir() || ignore(&path) {
        return;
    }

    let mut q: Vec<path::PathBuf> = Vec::new();
    for item in fs::read_dir(path).unwrap() {
        let f = item.unwrap(); // f is a DirEntry
        if f.path().is_dir() {
            q.push(f.path());
        };
        if f.path().matches(&pattern) {
            println!("{}", f.path().display_colour());
        }
    }

    for d in q {
        explore(&d, &pattern);
    }
}
