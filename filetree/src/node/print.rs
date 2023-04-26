use super::Node;

/// config settings to define custom printing behavior
pub struct Config {
    /// ignore files/folders beginning with a '.' like '.gitignore'
    pub ignore_dot: bool,               
    /// ignore all files (only folders)
    pub ignore_files: bool,             
    /// ignore empty folders
    pub ignore_empty_folders: bool,     
    /// only print till to the depth of n-folders
    pub with_max_depth: usize,          
    /// include (comulated) filesize
    pub show_filesize: bool,            
}

impl Node {
    /// custom printing of the node-format to the console
    pub fn print_with_depth(&self, depth: usize, max_depth: usize) {

        let children = self.children.iter();
        let lastchild = children.len();
        for (idx, child) in children.enumerate() {
            // optional skipping of .gitignore etc...
            if child.name.starts_with("."){
                continue;
            }
            for _i in 0..depth {
                print!("│{:>whitespace$}", "", whitespace = 2);
            }

            if idx+1==lastchild{
                println!("└──{}", child.name);
            } else {
                println!("├──{}", child.name);
            }


            // println!("─ {}", child.name);
            if child.is_dir && depth < max_depth {
                child.print_with_depth(depth + 1, max_depth)
            }
        }
    }
    pub fn print_all(&self){
        let max_depth= usize::MAX;
        self.print_with_depth(0, max_depth)

    }
}

