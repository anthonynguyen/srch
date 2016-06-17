use std::env;
use std::fs;

fn main() {
    let curdir = env::current_dir().unwrap();
    for item in fs::read_dir(curdir).unwrap() {
        let f = item.unwrap();
        println!("{}: {}", pchar(&f), f.path().file_name().unwrap().to_str().unwrap());
    }
}

fn pchar(path: &fs::DirEntry) -> char {
    match is_dir(path) {
        true => 'D',
        false => 'F',
    }
}

fn is_dir(path: &fs::DirEntry) -> bool {
    path.metadata().unwrap().is_dir()
}
