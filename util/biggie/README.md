# Biggie

This program generates a file with a given number of lines of randomly generated text.
The point is to make a really big file for testing other programs.
Run with `-h|--help` for usage:

```
$ cargo run -- --help
biggie 0.1.0
Ken Youens-Clark <kyclark@gmail.com>
Make big text files

USAGE:
    biggie [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --lines <LINES>     Number of lines [default: 100000]
    -o, --outfile <FILE>    Output filename [default: out]
```

## Author

Ken Youens-Clark <kyclark@gmail.com>
