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

### Installation

Navigate to this project's source directory and run

    cargo install

### License

`srch` is licensed under the MIT License. See the `LICENSE` file for details
