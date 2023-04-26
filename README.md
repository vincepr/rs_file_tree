# File Tree output as string, std-out etc

## possible flags to implement
- ignore `.files and .folders`
- ignore files
- only till depth `--depth 3`
- colorize folders in terminal
- exclude empty folders
- `out ./filepath` write it to file instead of console

## Goals
Functionality to produce formated tree of a path:
```
rs_file_tree
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
│   ├── default.json
│   ├── production.json
│   └── test.json
├── rustfmt.toml
├── src
│   ├── database.rs
│   ├── errors.rs
│   ├── lib
│   │   ├── authenticate_request.rs
│   │   ├── date.rs
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── to_object_id.rs
│   │   └── token.rs
│   ├── logger.rs
│   ├── main.rs
│   ├── models
│   │   ├── cat.rs
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── routes
│   │   ├── cat.rs
│   │   ├── mod.rs
│   │   ├── status.rs
│   │   └── user.rs
│   ├── settings.rs
│   └── tests
└── test.sh
```

- split into 2 workspaces 
    - 1 binary to create the binary for linux- (and possible win-) terminal
    - 1 library that will output a formated string.
- optional feature flags:
    - `-- folders`folders only
    - `--depth 2` limit depth


# Configuring multi-workspace structure with cargo Rust
## creating the project structure
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


```

 ├──depth1
│  ├──emptyfolder
│  ├──somefolder
│  │  ├──depth2.txt
│  ├──otherfolder
│  │  ├──stacked_folder
│  │   │  ├──deepest_file.txt
│  │  ├──otherdepth2
│  ├──depth1.txt
 ├──settings.txt
 ├──rootfile.txt
```