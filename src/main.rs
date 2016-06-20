extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use ansi_term::Colour;
use docopt::Docopt;

use std::collections;
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
            Ok(Colour::Purple.paint(s).to_string())
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
    let mut q: collections::VecDeque<path::PathBuf> = collections::VecDeque::new();
    let mut results = (0, 0);

    q.push_back(path.clone());
    while q.len() > 0 {
        let p = q.pop_front().unwrap();
        let r = explore(&mut q, &p, pattern);
        match r {
            Ok(n) => {
                results.0 += n.0;
                results.1 += n.1;
            },
            Err(_) => (),
        };
    }

    let (directories, files) = results;
    println!("Search results: {} directories, {} files", Colour::Cyan.bold().paint(directories.to_string()), Colour::Blue.bold().paint(files.to_string()));
}

fn explore(q: &mut collections::VecDeque<path::PathBuf>, path: &path::PathBuf, pattern: &String) -> std::io::Result<(i32, i32)> {
    if try!(ignore(path)) {
        return Ok((0, 0))
    }

    let mut directories = 0;
    let mut files = 0;

    for item in try!(fs::read_dir(path)) {
        let f = try!(item); // f is a DirEntry
        let dir: bool;
        if try!(f.path().is_dir()) {
            q.push_back(f.path());
            dir = true;
        } else {
            dir = false;
        }

        if f.path().matches(&pattern) {
            println!("{}", try!(f.path().display_colour()));
            if dir {
                directories += 1;
            } else {
                files += 1;
            }
        }
    }

    Ok((directories, files))
}
