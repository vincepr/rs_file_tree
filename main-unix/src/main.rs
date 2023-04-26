use std::{path::PathBuf, env, process};




fn main() {
    // reading in args:
    let path = parse_args().unwrap_or_else(|err|{
        eprintln!("input-Error: {err}");
        process::exit(1);
    });

    // creating the tree-structure of folders and files:
    let path = PathBuf::from(path);
    let nodes = filetree::walk(&path);
    if let Err(err) = nodes{
        eprintln!("walking-folder-Error: {err}");
        process::exit(1);
    } else{
        //running successfuly:

        println!("{:?}",nodes);
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
