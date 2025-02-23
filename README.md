# meow

A Rust implementation of `cat` with CLI argument parsing using [`clap`](https://crates.io/crates/clap).

## Installation

Clone the repository and build the project:

```sh
git clone https://github.com/kiemonbb/meow.git
cd meow
cargo build --release
```

## Usage

```sh
meow [FLAGS] [INPUT_FILES]...
```

### FLAGS:

- `-n`, `--number` - Show line numbers for all lines.
- `-b`, `--blank` - Do not show line numbers for blank lines.
- `-s`, `--sus` - Do not display error message if the program cannot find an input file
- `-h`, `--help` - Display help message.

### Examples:

1. To display a file, enter:
```sh
meow file
```
2. To concatenate several files, enter: 
```sh
meow file1 file2 > file3
```
3. To append one file to the end of another, enter: 

```sh
meow file1 >> file2
```
4. To add text to the end of a file, enter:
```sh
meow >> file
```

## TODO
- improve error messages and error handling
- add more flags
