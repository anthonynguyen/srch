extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use ansi_term::Colour;
use ansi_term::Style;
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
    explore(&curdir);
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

trait IsDir {
    fn is_dir(&self) -> bool;
}

impl IsDir for fs::DirEntry {
    fn is_dir(&self) -> bool {
        self.metadata().unwrap().is_dir()
    }
}

impl IsDir for path::PathBuf {
    fn is_dir(&self) -> bool {
        fs::metadata(self).unwrap().is_dir()
    }
}

trait DisplayColour {
    fn display_colour(&self, dstyle: Style, fstyle: Style, short: bool) -> String;
}

impl DisplayColour for path::PathBuf {
    fn display_colour(&self, dstyle: Style, fstyle: Style, short: bool) -> String {
        let style: Style;
        if self.is_dir() {
            style = dstyle;
        } else {
            style = fstyle;
        }

        if short {
            style.paint(self.file_name().unwrap().to_str().unwrap()).to_string()
        } else {
            style.paint(self.to_str().unwrap()).to_string()
        }
    }
}

fn explore(path: &path::PathBuf) -> () {
    if !path.is_dir() || ignore(&path) {
        return;
    }
    println!("{}:", path.display_colour(Colour::Yellow.bold(), Style::default(), false));

    let mut q: Vec<path::PathBuf> = Vec::new();
    for item in fs::read_dir(path).unwrap() {
        let f = item.unwrap(); // f is a DirEntry
        if f.is_dir() {
            q.push(f.path());
        };
        println!("{}", f.path().display_colour(Colour::Cyan.normal(), Style::default(), true));
    }

    print!("\n");

    for d in q {
        explore(&d);
    }
}
