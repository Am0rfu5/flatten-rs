use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use path_clean::PathClean;

#[derive(Debug)]
pub struct ExcludeItem(PathBuf);
#[derive(Debug)]
pub struct IncludeItem(PathBuf);

impl ExcludeItem {
    pub fn new(base_dir: &Path, path: PathBuf) -> Option<Self> {
        let full_path = base_dir.join(path);
        canonicalize(full_path).ok().map(|mut p| {
            let rel_path = p.strip_prefix(base_dir).unwrap_or(&p).to_path_buf();
            let pattern = format!("!{}", rel_path.display());
            // println!("Exclude pattern: {}", pattern);
            PathBuf::from(pattern)
        }).map(ExcludeItem)
    }
}

impl IncludeItem {
    pub fn new(base_dir: &Path, path: PathBuf) -> Option<Self> {
        let full_path = base_dir.join(path);
        canonicalize(full_path).ok().map(|p| {
            let rel_path = p.strip_prefix(base_dir).unwrap_or(&p).to_path_buf();
            PathBuf::from(rel_path)
        }).map(IncludeItem)
    }
}

#[derive(Debug)]
pub struct ExcludeList(pub Vec<PathBuf>);
#[derive(Debug)]
pub struct IncludeList(pub Vec<PathBuf>);

impl ExcludeList {
    pub fn new(base_dir: &Path, excludes: Vec<PathBuf>) -> Self {
        let mut list = Vec::new();

        // Add default excludes
        if let Some(git) = ExcludeItem::new(base_dir, PathBuf::from("flattenrs")) {
            list.push(git.0);
        }

        // Add user-specified excludes
        list.extend(excludes.into_iter().filter_map(|p| ExcludeItem::new(base_dir, p).map(|item| item.0)));

        ExcludeList(list)
    }

    pub fn contains(&self, path: &Path) -> bool {
        self.0.contains(&path.to_path_buf())
    }
}

impl IncludeList {
    pub fn new(base_dir: &Path, includes: Vec<PathBuf>) -> Self {
        let list: Vec<PathBuf> = includes.into_iter()
            .filter_map(|p| IncludeItem::new(base_dir, p).map(|item| item.0))
            .collect();

        IncludeList(list)
    }

    pub fn contains(&self, path: &Path) -> bool {
        self.0.contains(&path.to_path_buf())
    }
}

impl fmt::Display for ExcludeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExcludeList: [")?;
        for path in &self.0 {
            write!(f, "{}, ", path.display())?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for IncludeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IncludeList: [")?;
        for path in &self.0 {
            write!(f, "{}, ", path.display())?;
        }
        write!(f, "]")
    }
}

pub fn absolute_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

fn canonicalize<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    let mut components = path.components();
    let mut result = if path.is_absolute() {
        PathBuf::from("/")
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            std::path::Component::RootDir => {}
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::Normal(name) => {
                result.push(name);
            }
            _ => {}
        }
    }

    fs::canonicalize(result)
}
