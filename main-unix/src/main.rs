use std::{path::PathBuf, process};

use clap::Parser;
use filetree::Config;

// // Define available flags:
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Folder to dive into. Workdir is assumed if omitted.
    path: PathBuf,

    /// Ignore files/folders beginning with a '.' Like '.gitignore'
    #[arg(long, short, action)]
    dotignore: bool,

    /// Only display folders. Does not display any files
    #[arg(long, short, action)]
    folders: bool,

    /// How many layers deep to dive into folder structure
    #[arg(long, short)]
    maxdepth: Option<usize>,

    /// Display size in B, KB, MB, GB for folders and files (commulative)
    #[arg(long, short, action)]
    size: bool,
}

fn main() {
    // parse cli-Arguments with clap:
    let args = Args::parse();
    println!("{args:?}");
    let cfg = Config {
        ignore_dot: args.dotignore,
        ignore_files: args.folders,
        with_max_depth: args.maxdepth,
        show_filesize: args.size,
    };

    run(cfg, &args.path);
}

// running the filetree crate
fn run(cfg: Config, path: &PathBuf) {
    // creating the tree-structure of folders and files:
    let tree = filetree::new(path);
    if let Err(err) = tree {
        eprintln!("walking-folder-Error: {err}");
        process::exit(1);
    }
    // print out the created structure to the terminal:
    let tree = tree.unwrap();
    tree.print(cfg);
    process::exit(0);
}
