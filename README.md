# srch

`srch` is a simple command-line file search utility written in Rust.

    Usage:
        srch [options] [<path>] <pattern>
        srch --help
        srch --version

    Options:
        -h, --help              Show this help screen
        -v, --version           Show this program's version
        -i, --invisible         Also search inside directories starting with a . character
        -f, --filesonly         Only search filenames and NOT directory names
        -r, --regex             Treat <pattern> as a regular expression

### Notable Features

* Coloured output
* Friendly command line options
* Regex support

### Installation

Navigate to this project's source directory and run

    cargo install

### Performance

`srch`'s performance is considerably worse than `find`. For comparable queries, `find` is considerably faster than `srch`:

    $ time srch -ir / .*git.*
    ...
    srch -ir / .*git.*  2.31s user 32.85s system 69% cpu 50.684 total
    $ time find / -name *git*
    ...
    find / -name *git*  1.33s user 3.27s system 50% cpu 9.016 total

### License

`srch` is licensed under the MIT License. See the `LICENSE` file for details
