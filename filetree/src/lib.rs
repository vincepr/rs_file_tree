use std::{path::PathBuf, os::unix::prelude::MetadataExt, ffi::OsStr};


/// struct to hold files/folders in a tree like datastructure
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Node {
    name: String,
    children: Vec<Node>,
    is_dir: bool,
    size: u64,  // total size in bytes
}

/// Walks the file system starting from a folder-path
/// - returns a tree of Nodes representing the folders and files
pub fn walk(path_to_folder: &PathBuf) -> Result<Node, Box<dyn std::error::Error>> {
    // current folder:
    let mut root = Node{
        name: path_buf_to_str(path_to_folder),
        is_dir: true,
        children: vec![],
        size: 0,
    };

    // append children inside the current-dir:
    for dir_entry in  std::fs::read_dir(path_to_folder)? {
        let dir_entry = dir_entry?;
        let path = &dir_entry.path();
        let metadata = &std::fs::metadata(&path)?;
        let size = metadata.size();                 // TODO: will this work in windows && Unix?
        root.size += size;
        let node = match metadata.is_dir() {
            // current node is a file:
            false => Node {
                    name: path_buf_to_str(path),
                    is_dir: false,
                    children: vec![],
                    size,
                },
            // current node is a folder/directory:
            true => walk(path)?,
        };
        root.children.push(node);
    };
    // sort children (folders top) files later. all alphabetically
    root.children.sort_by(|a,b|{
        b.is_dir.cmp(&a.is_dir)
        // TODO: check if this will be sorted lexically for unix && win? or if we have to sort
    });
    Ok(root)
}


/// helper-fn. converts PathBuffer to a Folder- or File-Name String
fn path_buf_to_str(path_buf: &PathBuf) -> String {
    path_buf.file_name().unwrap_or(OsStr::new("noFilename")).to_string_lossy().into()
}

impl Node{
    /// get a formated string for the Node-structure
    pub fn formated_string(&self){
        println!("{:#?}",self);
    }
    fn print_with_depth(&self, depth: usize) {
        for child in self.children.iter(){
            for i in 0..depth{
                print!("{:>offset$}▕", "", offset=i);
                // if i == depth-1{
                //     print!("{:>offset$} └", "", offset=i+1);
                // }
            }

            println!("─ {}", child.name);
            if child.is_dir && depth >0 {
                child.print_with_depth(depth+1)
            }

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = PathBuf::from("./../test");
        let mut nodes = walk(&path).unwrap();
        //nodes.formated_string();
        nodes.print_with_depth(1);
    }
}


// let children = self.children.iter();
//         let lastchild = children.len();
//         for (idx, child) in children.enumerate(){
//             for i in 0..depth{
//                 if idx+1==lastchild{
//                     print!("{:>offset$}▕", "", offset=i);
//                     print!("└");
//                 } else {
//                     print!("{:>offset$}▕", "", offset=i+1);

//                 }
//             }
//             println!("─ {}", child.name);
//             if child.is_dir && depth >0 {
//                 child.print_with_depth(depth+1)
//             }

//         }