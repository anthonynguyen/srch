extern crate ansi_term;
extern crate docopt;
extern crate regex;
extern crate rustc_serialize;
extern crate time;

use ansi_term::Colour;
use docopt::Docopt;
use regex::Regex;

use std::{fs, io, process};
use std::collections::VecDeque;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;

const USAGE: &'static str = "
srch, a command-line file search utility written in Rust.

Usage:
    srch [options] [<path>] <pattern>
    srch --help
    srch --version

Options:
    -h, --help              Show this help screen
    -v, --version           Show this program's version
    -i, --invisible         Also search inside directories starting with a . character
    -f, --filesonly         Only search filenames and NOT directory names
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

struct Settings {
    pattern: Regex,
    path: String,

    invisible: bool,
    files_only: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .unwrap()
        .help(true)
        .version(Some(String::from("srch ") +
                      option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")))
        .decode()
        .unwrap_or_else(|e| e.exit());

    let settings = Settings {
        pattern: Regex::new(format!("^{}$", args.arg_pattern).as_str()).unwrap(),
        path: args.arg_path,
        invisible: args.flag_i || args.flag_invisible,
        files_only: args.flag_f || args.flag_filesonly,
    };

    let dir = PathBuf::from(&settings.path);
    if settings.path.is_empty() {
        let curdir = PathBuf::from(".");
        handle(&curdir, &settings);
    } else if !dir.exists() {
        println!("Invalid search path: {}",
                 Colour::Red.bold().paint(settings.path.as_str()));
        process::exit(1);
    } else {
        handle(&dir, &settings);
    }
}

fn ignore(path: &PathBuf, settings: &Settings) -> io::Result<bool> {
    if !path.is_dir() {
        return Ok(true);
    }

    let m = try!(path.symlink_metadata()).file_type();
    if m.is_symlink() || m.is_block_device() || m.is_char_device() || m.is_fifo() || m.is_socket() {
        return Ok(true);
    }

    if settings.invisible {
        return Ok(false);
    }

    let fname = path.file_name();
    if fname == None {
        return Ok(false);
    }

    let fname = fname.unwrap().to_str().unwrap();
    if fname.chars().next().unwrap() == '.' {
        return Ok(true);
    }

    Ok(false)
}

trait Matches {
    fn matches(&self, &Regex) -> bool;
}

impl Matches for fs::DirEntry {
    fn matches(&self, pattern: &Regex) -> bool {
        pattern.is_match(self.file_name().to_str().unwrap())
    }
}

trait DisplayColour {
    fn display_colour(&self) -> io::Result<String>;
}

impl DisplayColour for PathBuf {
    fn display_colour(&self) -> io::Result<String> {
        let s: &str = self.to_str().unwrap();

        if self.is_dir() {
            Ok(Colour::Purple.paint(s).to_string())
        } else {
            Ok(s.to_string())
        }
    }
}

struct SearchResults {
    directories: i32,
    files: i32,

    scanned: i32,
    pushed: i32,
}

fn handle(path: &PathBuf, settings: &Settings) -> () {
    let mut q: VecDeque<PathBuf> = VecDeque::new();
    let mut results = SearchResults {
        directories: 0,
        files: 0,
        scanned: 0,
        pushed: 1,
    };

    let pattern = &settings.pattern;

    let start = time::precise_time_ns();

    println!("Searching {} for {}",
             Colour::Yellow.bold().paint(path.to_str().unwrap()),
             Colour::Green.bold().paint(pattern.as_str()));
    q.push_back(path.clone());
    while q.len() > 0 {
        let p = q.pop_front().unwrap();
        match ignore(&p, settings) {
            Ok(i) => i && continue,
            Err(_) => continue,
        };

        match search(&mut q, &p, settings, &mut results) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    println!("Explored {} directories and searched {} objects, found {} directories and {} files",
             Colour::Yellow.bold().paint(results.pushed.to_string()),
             Colour::Yellow.bold().paint(results.scanned.to_string()),
             Colour::Green.bold().paint(results.directories.to_string()),
             Colour::Green.bold().paint(results.files.to_string()));

    let elapsedms = (time::precise_time_ns() - start) / 1000000;
    println!("Search took {}",
             Colour::Blue.bold().paint((elapsedms).to_string() + "ms"));
}

fn search(q: &mut VecDeque<PathBuf>,
          path: &PathBuf,
          settings: &Settings,
          results: &mut SearchResults)
          -> std::io::Result<()> {

    for item in try!(path.read_dir()) {
        let f = try!(item); // f is a DirEntry
        let dir = f.file_type().unwrap().is_dir();
        if dir {
            results.pushed += 1;
            q.push_back(f.path());
            if settings.files_only {
                continue;
            }
            results.scanned += 1;
        } else {
            results.scanned += 1;
        }

        if f.matches(&settings.pattern) {
            println!("{}", try!(f.path().display_colour()));
            if dir {
                results.directories += 1;
            } else {
                results.files += 1;
            }
        }
    }

    Ok(())
}
