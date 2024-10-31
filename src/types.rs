use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a file path to exclude during directory traversal.
/// `ExcludeItem` provides a convenient abstraction for defining
/// paths that should not be processed.
#[derive(Debug)]
pub struct ExcludeItem(PathBuf);

impl ExcludeItem {
    /// Constructs a new `ExcludeItem` from a base directory and path.
    /// Returns `None` if the path cannot be resolved.
    ///
    /// This method combines the base directory and path, then canonicalizes it to
    /// create an exclusion pattern.
    /// 
    /// # Arguments
    ///
    /// * `base_dir` - The base directory to which the path is relative.
    /// * `path` - The path to exclude.
    ///
    /// # Returns
    ///
    /// * `Some(ExcludeItem)` if the path is successfully canonicalized.
    /// * `None` if canonicalization fails.
    pub fn new(base_dir: &Path, path: PathBuf) -> Option<Self> {
        // Canonicalize the path to ensure a consistent absolute reference.
        // This helps avoid mismatches caused by relative paths or symbolic links.
        let full_path = base_dir.join(path);
        canonicalize(full_path).ok().map(|p| {
            let rel_path = p.strip_prefix(base_dir).unwrap_or(&p).to_path_buf();
            let pattern = format!("!{}", rel_path.display());
            PathBuf::from(pattern)
        }).map(ExcludeItem)
    }
}

/// Represents a file path to explicitly include during directory traversal.
/// Similar to `ExcludeItem`, `IncludeItem` is used to define paths that should
/// be included in the flattened directory output.
#[derive(Debug)]
pub struct IncludeItem(PathBuf);

impl IncludeItem {
    /// Constructs a new `IncludeItem` from a base directory and path.
    /// Returns `None` if the path cannot be resolved.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory to which the path is relative.
    /// * `path` - The path to include.
    ///
    /// # Returns
    ///
    /// `Some(IncludeItem)` if the path is valid, `None` otherwise.
    pub fn new(base_dir: &Path, path: PathBuf) -> Option<Self> {
        let full_path = base_dir.join(path);
        canonicalize(full_path).ok().map(|p| {
            let rel_path = p.strip_prefix(base_dir).unwrap_or(&p).to_path_buf();
            PathBuf::from(rel_path)
        }).map(IncludeItem)
    }
}

/// A collection of paths to exclude during directory traversal.
///
/// `ExcludeList` is used to specify a list of paths that should be ignored,
/// providing methods to check if a path is within the excluded items.
#[derive(Debug)]
pub struct ExcludeList(pub Vec<PathBuf>);


impl ExcludeList {
    /// Constructs a new `ExcludeList` based on a set of exclude paths.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory for resolving exclude paths.
    /// * `excludes` - A vector of relative paths to exclude.
    pub fn new(base_dir: &Path, excludes: Vec<PathBuf>) -> Self {
        let mut list = Vec::new();

        // Add default excludes
        // list.push(ExcludeItem::new(base_dir, PathBuf::from("flatten*")));
        if let Some(default_excludes) = ExcludeItem::new(base_dir, PathBuf::from("flatten")) {
            list.push(default_excludes.0);
        }

        // Add user-specified excludes
        list.extend(excludes.into_iter().filter_map(|p| ExcludeItem::new(base_dir, p).map(|item| item.0)));

        ExcludeList(list)
    }

    // @TODO: Implement contains method or remove it
    // Is this for testing or library?
    /// Checks if a specific path is within the exclusion list.
    /// 
    /// This method is used to verify if a path should be ignored during directory
    /// traversal, based on predefined exclusion criteria.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path to check for exclusion.
    /// 
    /// # Returns
    /// 
    /// * `true` if the path is in the exclusion list, `false` otherwise.
    #[allow(dead_code)]
    pub fn contains(&self, path: &Path) -> bool {
        // Converts `path` to `PathBuf` for easier matching with stored exclusion paths
        self.0.contains(&path.to_path_buf())
    }
}

/// A collection of paths to include during directory traversal.
///
/// `IncludeList` is used to specify a list of paths that should be
/// explicitly included, even if they would otherwise be excluded.
#[derive(Debug)]
pub struct IncludeList(pub Vec<PathBuf>);

impl IncludeList {
    /// Constructs a new `IncludeList` based on a set of include paths.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory for resolving include paths.
    /// * `includes` - A vector of relative paths to include.
    pub fn new(base_dir: &Path, includes: Vec<PathBuf>) -> Self {
        let list: Vec<PathBuf> = includes.into_iter()
            .filter_map(|p| IncludeItem::new(base_dir, p).map(|item| item.0))
            .collect();

        IncludeList(list)
    }
    
    // @TODO: Implement contains method or remove it
    /// Checks if a specific path is within the exclusion list.
    /// 
    /// This method is used to verify if a path should be ignored during directory
    /// traversal, based on predefined exclusion criteria.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path to check for exclusion.
    /// 
    /// # Returns
    /// 
    /// * `true` if the path is in the exclusion list, `false` otherwise.
    #[allow(dead_code)]
    pub fn contains(&self, path: &Path) -> bool {
        // Converts `path` to `PathBuf` for easier matching with stored inclusion paths
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

/// Generates a canonicalized path to ensure consistency and prevent
/// mismatches due to relative paths or symbolic links.
/// 
/// This function resolves `..`, `.`, and symbolic links in the given path,
/// producing an absolute and normalized path. It’s particularly useful
/// for ensuring that inclusion/exclusion rules are reliably applied.
/// 
/// # Arguments
/// 
/// * `path` - The path to be canonicalized.
/// 
/// # Returns
/// 
/// * `Ok(PathBuf)` - The canonicalized path on success.
/// * `Err(io::Error)` - An error if the path cannot be resolved.
///
/// # Errors
///
/// Returns an error if the path cannot be accessed, likely due to missing
/// permissions or invalid path segments.
fn canonicalize<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    let components = path.components();
    
    // Start from root if path is absolute, otherwise initialize with a blank PathBuf
    let mut result = if path.is_absolute() {
        PathBuf::from("/")
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            // Root and current directory components don't alter the path structure
            std::path::Component::RootDir => {}
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                // Remove the last component for each `..` in the path
                result.pop();
            }
            // Normal component represents a directory or file name to append
            std::path::Component::Normal(name) => {
                result.push(name);
            }
            _ => {}
        }
    }

    // Perform the final canonicalization to resolve symbolic links and normalize the path
    fs::canonicalize(result)
}
