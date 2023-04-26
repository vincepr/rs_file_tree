use std::{path::PathBuf, process};

use clap::{Parser, ArgAction};
use filetree::Config;

// // Define available flags:
#[derive(Parser, Debug)]
#[command(
    version, about="like 'tree': quickly get a file-tree representation for documentations")]
struct Args {
    /// Folder to dive into. Workdir is assumed if omitted.
    path: Option<PathBuf>,

    /// ignore an exact foldername/filename.
    #[arg(long, short, default_value="target", value_name = "FOLDER")]
    ignore: Option<String>,

    /// Include files/folders beginning with a '.' Like '.gitignore'
    #[arg(long, short, action=ArgAction::SetFalse)]
    dotignore: bool,

    /// Only display folders. Does not display any files
    #[arg(long, short, action)]
    folders: bool,

    /// How many layers deep to dive into folder structure
    #[arg(long, short, value_name = "uint32")]
    maxdepth: Option<usize>,

    /// Display size in B, KB, MB, GB for folders and files (commulative)
    #[arg(long, short, action)]
    size: bool,
}

fn main() {
    // parse cli-Arguments with clap:
    let args = Args::parse();
    let cfg = Config {
        ignore: args.ignore,
        ignore_dot: args.dotignore,
        ignore_files: args.folders,
        with_max_depth: args.maxdepth,
        show_filesize: args.size,
    };

    // if no path is set we assume the current working directory
    let path = args.path.unwrap_or(get_current_working_dir().unwrap());

    run(cfg, &path);
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

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    std::env::current_dir()
}