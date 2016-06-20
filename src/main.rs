extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Style;

use std::fs;
use std::path;

fn main() {
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
