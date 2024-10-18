use shellexpand;
use std::fmt;
use std::path::PathBuf;
use std::vec::IntoIter;
use std::{fs, io};

pub struct TreeClimber {
    path: String,
    all: bool,
}

impl Default for TreeClimber {
    fn default() -> TreeClimber {
        TreeClimber {
            path: String::from("."),
            all: false,
        }
    }
}

impl TreeClimber {
    pub fn new() -> TreeClimber {
        Default::default()
    }

    pub fn path(mut self, path: &str) -> TreeClimber {
        self.path = path.to_string();
        self
    }

    pub fn all(mut self, all: bool) -> TreeClimber {
        self.all = all;
        self
    }

    pub fn climb(self) -> io::Result<Tree> {
        // Perform shell expansion
        let path = shellexpand::tilde(&self.path).to_string();
        let path = PathBuf::from(path);

        // Build paths vector
        let mut paths: Vec<PathBuf> = Vec::new();
        recurse_dirs(&mut paths, path, self.all)?;

        Ok(Tree { paths })
    }
}

pub struct Tree {
    paths: Vec<PathBuf>,
}

impl IntoIterator for Tree {
    type Item = PathBuf;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

// TODO: make this a pretty-print, like the "tree" command
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.paths {
            let disp = p.display();
            write!(f, "{}\n", disp)?;
        }

        Ok(())
    }
}

fn recurse_dirs(paths: &mut Vec<PathBuf>, path: PathBuf, all: bool) -> io::Result<()> {
    // Sort entries in dir
    let mut entries = fs::read_dir(&path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    paths.push(path);

    // Iterate over sorted entries
    for entry in entries {
        if entry.is_dir() {
            // Skip hidden paths if desired
            if is_hidden(&entry) && !all {
                continue;
            }
            recurse_dirs(paths, entry, all)?;
        }
    }
    Ok(())
}

fn is_hidden(path: &PathBuf) -> bool {
    if let Some(n) = path.file_name() {
        if let Some(n_str) = n.to_str() {
            return n_str.starts_with(".");
        }
    }
    false
}
