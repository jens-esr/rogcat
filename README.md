[![Build Status](https://travis-ci.org/flxo/rogcat.png)](https://travis-ci.org/flxo/rogcat)
# rogcat


A ``adb logcat`` wrapper with colored output and filter capabilities.

![Screenshot](/screenshot.png)

## Usage

Without any argument rogcat starts up adb logcat and displays the output in a nice way. 
Not all errors are wrapped with a nice and readable explanation, so please be prepared to see something strange.

```
rogcat 0.1.1
Felix Obenhuber <f.obenhuber@gmail.com>
A logcat wrapper

USAGE:
    rogcat [FLAGS] [OPTIONS] [--] [COMMAND]

FLAGS:
        --disable-tag-shortening    Disable shortening of tag in human format
        --disable-color-output      Monochrome output
    -S                              Output statistics
    -c                              Clear (flush) the entire log and exit
    -g                              Get the size of the log's ring buffer and exit
    -h, --help                      Prints help information
    -V, --version                   Prints version information

OPTIONS:
        --adb <ADB BINARY>    Path to adb
        --file <FILE>         Write to file
        --input <INPUT>       Read from file instead of command
        --level <LEVEL>       Minumum loglevel
        --msg <FILTER>...     Message filters in RE2
        --tag <FILTER>...     Tag filters in RE2

ARGS:
    <COMMAND>    Optional command to run and capture stdout
```

## Todos

* Configurable terminal
* Error messages instead of raw panics
* Extension of in and output formats
* Optimization of format implementation
* Ring buffer like file output
* ...

## Bugs

There are plenty. Report them on GitHub, or - even better - open a pull request.

## Licensing

Rogcat is open source software; see ``COPYING`` for details.

## Author

Rogcat was written by Felix Obenhuber
