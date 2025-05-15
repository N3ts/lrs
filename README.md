# **lrs** - A reimplementation of ls in Rust
**lrs** is a Rust-reimplementation of the GNU version of the classic terminal command **coreutils/ls**, written for 64-bit Linux desktops. 

No support for Mac/Windows/etc.!

## Intention
Originally intended as a project for university, lrs reimplements some of the most basic features one would expect from GNU's ls. 

The idea behind it was to implement a memory safe version of the iterative recursion function of the coreutils-counterpart. Therefore **lrs** is compiled using safe Rust for the basic logic behind the `-R`-argument and other features such as the *column* or *long* format output.  

For this project I mostly made use of the standard Library. Error conversion, time stamps, calculation of unicode character widths and argument parsing are implemented using Cargo-Crates. Whether these are safe, is dependent on their implementation. Check `src/Cargo.toml` for these dependencies.

## Notice 
Please note that this is my first rust program. Even though it is based on my analysis of the original GNU/ls-code, **lrs** is currently taking baby steps and does not implement many features - except for some of the most common ones. The project is not very clean and still needs to be tinkered with and has to refactored quite a bit. So far it has been tested for stability, but not necessarily for correct output, as there are many types of files, that could somehow be broken, that I haven't accounted for. So do not expect this to work as intended. It does what it should at a minimum, but it seems to do it well and I'm still proud of it, for what it is.

**Please let me know if you find any bugs**

## Features
- Outputs multiple files passed in by the command line
- Outputs directory entries
- Column and long (`-l`) format output
- Iterative directory recursion (`-R`)
- Dereferences symbolic links (`-l`)
- Show hidden "."-files
- Outputs help (`-h`) 
- Compatible with ls arguments of implemented features 
- Standard Color output

## Missing features
- Does not output "."- and ".."-entries, as those are not output by std::fs::ReadDir
- Even though column output is supported as standard output, `-C` is currently not a supported argument
- The possible Columns layouts are always recalculated instead of being cached and expanded
  - This is mostly due to me focusing on getting the project working
  - It works for everyday use, with a common amount of files per directory
  - For speed purposes I recommend using `-l` 
- Color output cannot be turned off, therefore no arguments regarding it are defined
  - Ergo: No support for terminals that don't support color output
- No color support for file types other than executables, directories and symbolic links
- Parsing of `LS_COLORS` system variable is also not supported
- Every other ls feature not listed in the above is also not included
- No guarantee of POSIX-conformity!

## TODO:
- [ ] Missing Features 
- [ ] Documentation

## Requirements
- [Rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) 

## Build 
```bash 
$ git clone https://github.com/N3ts/lrs.git ~/tmp/lrs
$ cd ~/tmp/lrs 
$ cargo build --release
```
## Installation
Run the following commands in the project directory:

### Install script
Make the install script executable and run it:
```
$ chmod u+x install.sh
$ ./install.sh
```
The install script will execute `sudo` to move the binary to `/usr/local/bin`, to install it globally.

### Manual Install 
Just move the binary to either of the following directories:

#### System wide 
``` 
# cp target/release/lrs /usr/local/bin/
```
#### Local 
```
$ cp target/release/lrs ~/.local/bin/
```
Make sure to add `~/.local/bin` to your `$PATH` variable. Using your favorite editor, in your ~/.bashrc add: 

``` 
export PATH=$PATH:~/.local/bin
``` 
Source your `.bashrc` and/or restart your terminal adding to `$PATH`, or else the command won't execute!

## Execution 
Just run the following command: 
```
$ lrs
``` 
For an Overview of available arguments execute the Command with the `-h` flag: 
```
$ lrs -h
```  

## License 
GPL 3.0