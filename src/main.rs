extern crate ansi_term;

use ansi_term::Colour;

use std::env;
use std::fs;
use std::path;
use std::vec;

fn main() {
    let curdir = env::current_dir().unwrap();
    explore(&curdir);
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
    fn display_colour(&self) -> String;
}

impl DisplayColour for path::PathBuf {
    fn display_colour(&self) -> String {
        match self.is_dir() {
            true => Colour::Yellow.bold().paint(self.to_str().unwrap()).to_string(),
            false => String::from(self.to_str().unwrap()),
        }
    }
}

fn explore(path: &path::PathBuf) -> () {
    println!("{}", path.display_colour());

    if !path.is_dir() {
        return;
    }

    let mut q: Vec<path::PathBuf> = Vec::new();
    for item in fs::read_dir(path).unwrap() {
        let f = item.unwrap(); // f is a DirEntry
        match f.is_dir() {
            true => {
                q.push(f.path());
            },
            false => {
                explore(&f.path());
            },
        };
    }

    for d in q {
        explore(&d);
    }
}
