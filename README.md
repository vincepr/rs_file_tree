# File Tree parser
Quick way to parse file tree of projects for Documentation. (good default settings for Rust Projects like to ignore '/target' and to ignore '.git' ...)

## Usage
- will run on current path as failback (if no path is provied via argument)
- runs with `--ignore target` as default to ignore the Rust default folder for bulding.
- for more optional flags see below:
```
Usage: rs_file_tree [OPTIONS] [PATH]

Arguments:
  [PATH]  Folder to dive into. Workdir is assumed if omitted

Options:
  -i, --ignore <FOLDER>    ignore an exact foldername/filename [default: target]
  -d, --dotignore          Include files/folders beginning with a '.' Like '.gitignore'
  -f, --folders            Only display folders. Does not display any files
  -m, --maxdepth <uint32>  How many layers deep to dive into folder structure
  -s, --size               Display size in B, KB, MB, GB for folders and files (commulative)
  -h, --help               Print help
  -V, --version            Print version
```

## Output:
Functionality to produce formated tree of a path:
```
vincepr@linux:~/rust/rs_file_tree$ ./bin/rs_file_tree
rs_file_tree
├──main-unix
│  ├──src
│  │  └──main.rs
│  └──Cargo.toml
├──filetree
│  ├──src
│  │  ├──node
│  │  │  ├──print.rs
│  │  │  └──mod.rs
│  │  └──lib.rs
│  └──Cargo.toml
├──bin
│  └──main-unix
├──demo_file_tree.gif
├──Cargo.toml
├──Cargo.lock
└──README.md
```
![Demo gif](./demo_file_tree.gif)

## Building it yourself:
- with rust and cargo installed run: `cargo build --release --package main-unix`
- afterwards one can copy the binary from: `target/release/main-unix`


# Notes for my future self
## Configuring multi-workspace structure with cargo Rust
### creating the project structure
```
cargo new rs_file_tree --lib
cd rs_file_tree
cargo new filetree --lib
cargo new main-unix
cargo new main-win
```
In the root project's Cargo.toml we delete everything but the workspace definitions/paths:
```
[workspace]
members=[
    "main-unix",
    "filetree",
]
```
- now we can already `cargo build`
### using code from filetree crates
In our main crates we import our library:
```
[dependencies]
filetree = { path = "../filetree" }
```

## Adding Clap crate to parse Args
activate derive-mode-macros with the feature flag:
- `cargo add clap --features derive`