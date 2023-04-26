use std::{path::PathBuf, env, process};

use filetree::Config;




fn main() {
    // reading in args:
    let path = parse_args().unwrap_or_else(|err|{
        eprintln!("input-Error: {err}");
        process::exit(1);
    });

    let cfg = Config{
        ignore_dot: todo!(),
        ignore_files: todo!(),
        ignore_empty_folders: todo!(),
        with_max_depth: todo!(),
        show_filesize: todo!(),
    };

    // creating the tree-structure of folders and files:
    let path = PathBuf::from(path);
    let tree = filetree::new(&path);
    if let Err(err) = tree{
        eprintln!("walking-folder-Error: {err}");
        process::exit(1);
    } else{
        //running successfuly:
        let tree = tree.unwrap();
        tree.print_all();
        //println!("{:?}",tree);
    }
}


// comand line parsing:
fn parse_args() -> Result<String, &'static str> {
    let mut args = env::args();
    if args.len() != 2 {
        return Err("need exactly one Argument that points to the folder to search");
    }
    args.next();
    let path = args.next().unwrap();
    Ok(path)
}
