# excel2txt-rust

Rust implementation of excel2txt

```
excel2txt 0.1.0
Ken Youens-Clark <kyclark@gmail.com>
Export Excel workbooks into delimited text files

USAGE:
    excel2txt [FLAGS] [OPTIONS] --file <FILE>

FLAGS:
    -h, --help         Prints help information
    -m, --mkdirs       Make output directory for each input file
    -n, --normalize    Normalize headers
    -V, --version      Prints version information

OPTIONS:
    -d, --delimiter <DELIM>    Delimiter for output files [default: 	]
    -f, --file <FILE>          File input
    -o, --outdir <DIR>         Output directory [default: out]
```

* The program accepts one or more Excel workbooks as the `-f|--file` argument.
* Each worksheet will be exported to a text file delimited by the `-d|--delimiter` which defaults to the tab character.
* All files will be written to the `-o|--outdir` directory which defaults to "out".
* The `-m|--mkdirs` option will indicate that each workbook/Excel file will be placed into a separate directory in the output directory.
* The `-n|--normalize` flag will alter the headers of each output file to lowercase values and remove non-alphanumeric characters or the underscore. This will also break "CamelCase" values into "snake_case." (This same normalization will be used to create the output file names so as to avoid any possibility of creating output files with illegal or difficult characters.)
* The `-V|--version` will cause the program to print the current version and exit.

## Author

Ken Youens-Clark <kyclark@gmail.com>

Note: this is a Rust port of [excel2txt](https://github.com/kyclark/excel2txt-py) that I wrote in Python.

Special thanks to Huo Linhe's [xlsx2csv](https://github.com/zitsen/xlsx2csv.rs).
