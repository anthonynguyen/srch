extern crate ansi_term;
extern crate docopt;
extern crate rustc_serialize;

use ansi_term::Colour;
use docopt::Docopt;

use std::collections;
use std::io;
use std::path;
use std::process;

use std::os::unix::fs::FileTypeExt;

const USAGE: &'static str = "
srch, a command-line file search utility written in Rust.

Usage:
    srch [options] [<path>] <pattern>
    srch --help
    srch --version

Options:
    -h, --help        Show this help screen
    -v, --version     Show this program's version
    -i, --invisible   Also search inside directories starting with a . character
    -f, --filesonly   Only search filenames and NOT directory names
";

#[derive(RustcDecodable)]
struct Args {
    arg_pattern: String,
    arg_path: String,

    flag_i: bool,
    flag_invisible: bool,

    flag_f: bool,
    flag_filesonly: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).unwrap()
                            .help(true)
                            .version(Some(String::from("srch, version ") + option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")))
                            .decode()
                            .unwrap_or_else(|e| e.exit());

    let dir = path::PathBuf::from(&args.arg_path);
    if args.arg_path.is_empty() {
        let curdir = path::PathBuf::from(".");
        handle(&curdir, &args);
    } else if !dir.exists() {
        println!("Invalid search path: {}", Colour::Red.bold().paint(args.arg_path.as_str()));
        process::exit(1);
    } else {
        handle(&dir, &args);
    }
}

fn ignore (path: &path::PathBuf, args: &Args) -> io::Result<bool> {
    if !path.is_dir() {
        return Ok(true)
    }

    let m = try!(path.symlink_metadata()).file_type();
    if m.is_symlink() || m.is_block_device() || m.is_char_device() || m.is_fifo() || m.is_socket() {
        return Ok(true)
    }

    if args.flag_i || args.flag_invisible {
        return Ok(false)
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
    fn display_colour(&self) -> io::Result<String>;
    fn matches(&self, pattern: &String) -> bool;
}

impl Misc for path::PathBuf {
    fn display_colour(&self) -> io::Result<String> {
        let s: &str = self.to_str().unwrap();

        if self.is_dir() {
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

fn handle(path: &path::PathBuf, args: &Args) -> () {
    let mut q: collections::VecDeque<path::PathBuf> = collections::VecDeque::new();
    let mut results = (0, 0);

    let pattern = &args.arg_pattern;

    println!("Searching {} for {}", Colour::Yellow.bold().paint(path.to_str().unwrap()), Colour::Green.bold().paint(pattern.as_str()));
    q.push_back(path.clone());
    while q.len() > 0 {
        let p = q.pop_front().unwrap();
        let i = ignore(&p, args);
        match i {
            Ok(tf) => {
                if tf {
                    continue;
                }
            },
            Err(_) => continue,
        };

        let r = explore(&mut q, &p, pattern, args.flag_f || args.flag_filesonly);
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

fn explore(q: &mut collections::VecDeque<path::PathBuf>, path: &path::PathBuf, pattern: &String, ignore_dirs: bool) -> std::io::Result<(i32, i32)> {
    let mut directories = 0;
    let mut files = 0;

    for item in try!(path.read_dir()) {
        let f = try!(item); // f is a DirEntry
        let dir: bool;
        if f.path().is_dir() {
            q.push_back(f.path());
            if ignore_dirs {
                continue;
            }
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
