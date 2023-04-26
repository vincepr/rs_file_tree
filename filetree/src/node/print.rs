use super::Node;

/// config settings to define custom printing behavior
pub struct Config {
    /// ignore files/folders beginning with a '.' like '.gitignore'
    pub ignore_dot: bool,

    /// ignore all files (only folders)
    pub ignore_files: bool,

    /// only print till to the depth of n-folders. 0 -> root only, 1 -> open 1 folder deepr ...
    pub with_max_depth: Option<usize>,

    /// include (comulated) filesize
    pub show_filesize: bool,

    /// optional: ignore a whole filename/foldername (example '--ignore target' for rust)
    pub ignore: Option<String>,
}

impl Node {
    /// custom printing of the node-format to the console
    pub fn print(&self, cfg: Config) {
        println!("{}", self.name);
        // print everything if no explicit max depth is set:
        let max_depth = match cfg.with_max_depth {
            None => usize::MAX,
            Some(n) => n,
        };
        self.print_with_depth(0, &cfg, max_depth);
    }

    // traverses the node tree recursively till max_depth is reached
    fn print_with_depth(&self, depth: usize, cfg: &Config, max_depth: usize) {
        let children = self.children.iter();
        let lastchild = children.len();
        for (idx, child) in children.enumerate() {
            // optional skipping of .gitignore etc...
            if cfg.ignore_dot && child.name.starts_with(".") {
                continue;
            }
            // optional skipping of all files (showing folders only)
            if cfg.ignore_files && !child.is_dir {
                continue;
            }
            if let Some(ignore) = &cfg.ignore {
                if *ignore == child.name{
                    continue;
                }
            }

            // 'whitespaces and |'formating before the node
            for _i in 0..depth {
                print!("│{:>whitespace$}", "", whitespace = 2);
            }
            // print the node out
            child.print_node_name(idx + 1 == lastchild, cfg);

            // if node is a foder we go deeper
            if child.is_dir && depth < max_depth {
                child.print_with_depth(depth + 1, cfg, max_depth)
            }
        }
    }

    // print name formated like: "├── testfolder   524 MB "
    fn print_node_name(&self, is_lastchild: bool, cfg: &Config) {
        let filesize = match cfg.show_filesize {
            true => format!("   {}", format_size(self.size)),
            false => "".to_string(),
        };
        match is_lastchild {
            true => println!("└──{}{}", self.name, filesize),
            false => println!("├──{}{}", self.name, filesize),
        }
    }
}

// convert from bytes to 'xx KB', 'xx MB', 'xx GB'...
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024; // const (KB,MB) = (1,2) not viable in Rust?
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;
    const PB: u64 = 1024 * TB;
    match bytes {
        0..=KB => return format!("{} B", bytes),
        ..=MB => return format!("{} KB", bytes / KB),
        ..=GB => return format!("{} MB", bytes / MB),
        ..=TB => return format!("{} GB", bytes / GB),
        ..=PB => return format!("{} TB", bytes / TB),
        _ => return format!("{} PB", bytes / PB),
    }
}
