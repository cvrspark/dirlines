use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

const IGNORED_DIRS: &[&str] = &["target", "build", "node_modules", ".git", ".cache"];

pub struct File {
    pub path: PathBuf,
    pub content: String,
}

impl File {
    pub fn load_content(&mut self) -> io::Result<()> {
        self.content = fs::read_to_string(&self.path)?;
        Ok(())
    }
}

pub struct Workspace {
    pub path: PathBuf,
    pub files: Vec<File>,
}

impl Workspace {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            files: Vec::new(),
        }
    }

    pub fn load_files(&mut self) -> io::Result<()> {
        self.files.clear();
        let root = self.path.clone();
        self.scan_dir(&root)
    }

    fn scan_dir(&mut self, dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if IGNORED_DIRS.contains(&name) {
                    continue;
                }
            }

            if path.is_dir() {
                self.scan_dir(&path)?;
            } else if path.is_file() {
                let mut file = File {
                    path,
                    content: String::new(),
                };
                if file.load_content().is_ok() {
                    self.files.push(file);
                }
            }
        }
        Ok(())
    }
}