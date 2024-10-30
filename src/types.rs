use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a path that should be excluded from directory traversal.
#[derive(Debug)]
pub struct ExcludeItem(PathBuf);

impl ExcludeItem {
    /// Creates a new `ExcludeItem` instance from a given base directory and path.
    ///
    /// This method combines the base directory and path, then canonicalizes it to
    /// create an exclusion pattern.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory where the path is rooted.
    /// * `path` - A relative path to be excluded.
    ///
    /// # Returns
    ///
    /// * `Some(ExcludeItem)` if the path is successfully canonicalized.
    /// * `None` if canonicalization fails.

    pub fn new(base_dir: &Path, path: PathBuf) -> Option<Self> {
        let full_path = base_dir.join(path);
        canonicalize(full_path).ok().map(|p| {
            let rel_path = p.strip_prefix(base_dir).unwrap_or(&p).to_path_buf();
            let pattern = format!("!{}", rel_path.display());
            PathBuf::from(pattern)
        }).map(ExcludeItem)
    }
}

/// Represents a path that should be included explicitly in directory traversal.
#[derive(Debug)]
pub struct IncludeItem(PathBuf);

impl IncludeItem {
    /// Creates a new `IncludeItem` instance from a given base directory and path.
    ///
    /// This method canonicalizes the provided path to ensure valid inclusion.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory where the path is rooted.
    /// * `path` - A relative path to be included.
    ///
    /// # Returns
    ///
    /// * `Some(IncludeItem)` if the path is successfully canonicalized.
    /// * `None` if canonicalization fails.
    pub fn new(base_dir: &Path, path: PathBuf) -> Option<Self> {
        let full_path = base_dir.join(path);
        canonicalize(full_path).ok().map(|p| {
            let rel_path = p.strip_prefix(base_dir).unwrap_or(&p).to_path_buf();
            PathBuf::from(rel_path)
        }).map(IncludeItem)
    }
}

/// A list of paths to be excluded from directory traversal.
#[derive(Debug)]
pub struct ExcludeList(pub Vec<PathBuf>);


impl ExcludeList {
    /// Constructs a new `ExcludeList` instance with a default exclusion and optional
    /// user-specified exclusions.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory where exclusions are rooted.
    /// * `excludes` - A vector of relative paths to exclude.
    pub fn new(base_dir: &Path, excludes: Vec<PathBuf>) -> Self {
        let mut list = Vec::new();

        // Add default excludes
        // list.push(ExcludeItem::new(base_dir, PathBuf::from("flattenrs*")));
        if let Some(default_excludes) = ExcludeItem::new(base_dir, PathBuf::from("flattenrs")) {
            list.push(default_excludes.0);
        }

        // Add user-specified excludes
        list.extend(excludes.into_iter().filter_map(|p| ExcludeItem::new(base_dir, p).map(|item| item.0)));

        ExcludeList(list)
    }

    /// Checks if a path is present in the exclude list.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check against the exclude list.
    ///
    /// # Returns
    ///
    /// `true` if the path is excluded, `false` otherwise.
    pub fn contains(&self, path: &Path) -> bool {
        self.0.contains(&path.to_path_buf())
    }
}

#[derive(Debug)]
pub struct IncludeList(pub Vec<PathBuf>);

impl IncludeList {
    /// Constructs a new `IncludeList` instance with user-specified paths.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory where inclusions are rooted.
    /// * `includes` - A vector of relative paths to include.
    pub fn new(base_dir: &Path, includes: Vec<PathBuf>) -> Self {
        let list: Vec<PathBuf> = includes.into_iter()
            .filter_map(|p| IncludeItem::new(base_dir, p).map(|item| item.0))
            .collect();

        IncludeList(list)
    }
    
    /// Checks if a path is present in the include list.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check against the include list.
    ///
    /// # Returns
    ///
    /// `true` if the path is included, `false` otherwise.
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

fn canonicalize<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    let components = path.components();
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
