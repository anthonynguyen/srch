extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use ansi_term::Colour;
use docopt::Docopt;

use std::fs;
use std::io;
use std::path;

use std::os::unix::fs::FileTypeExt;

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

#[derive(RustcDecodable)]
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
    handle(&curdir, &args.arg_pattern);
}

fn ignore (path: &path::PathBuf) -> io::Result<bool> {
    if !try!(path.is_dir()) {
        return Ok(true)
    }

    let m = try!(fs::symlink_metadata(path)).file_type();
    if m.is_symlink() || m.is_block_device() || m.is_char_device() || m.is_fifo() || m.is_socket() {
        return Ok(true)
    }

    let fname = path.file_name();
    if fname == None {
        return Ok(false)
    }

    let fname = fname.unwrap().to_str().unwrap();
    if fname.chars().next().unwrap() == '.' {
        return Ok(true)
    }

    Ok(false)
}

trait Misc {
    fn is_dir(&self) -> io::Result<bool>;
    fn display_colour(&self) -> io::Result<String>;
    fn matches(&self, pattern: &String) -> bool;
}

impl Misc for path::PathBuf {
    fn is_dir(&self) -> io::Result<bool> {
        let m = try!(fs::metadata(self));
        Ok(m.is_dir())
    }

    fn display_colour(&self) -> io::Result<String> {
        let s: &str = self.to_str().unwrap();

        if try!(self.is_dir()) {
            Ok(Colour::Yellow.bold().paint(s).to_string())
        } else {
            Ok(s.to_string())
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

fn handle(path: &path::PathBuf, pattern: &String) -> () {
    match explore(path, pattern) {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn explore(path: &path::PathBuf, pattern: &String) -> std::io::Result<()> {
    if try!(ignore(path)) {
        return Ok(())
    }

    let mut q: Vec<path::PathBuf> = Vec::new();
    for item in try!(fs::read_dir(path)) {
        let f = try!(item); // f is a DirEntry
        if try!(f.path().is_dir()) {
            q.push(f.path());
        };
        if f.path().matches(&pattern) {
            println!("{}", try!(f.path().display_colour()));
        }
    }

    for d in q {
        handle(&d, pattern);
    }
    Ok(())
}
