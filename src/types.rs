// src/types.rs
// use std::env;
// use std::io;
// use path_clean::PathClean;
// use std::path::{Path, PathBuf};

// pub struct ExcludeItem(Path);
// pub struct IncludeItem(Path);

// impl ExcludeItem {
//     pub fn new(base_dir: &Path, path: Path) -> Option<Self> {
//         // absolute_path(base_dir.join(path)).ok().map(ExcludeItem)
//     }
// }

// impl IncludeItem {
//     pub fn new(path: Path) -> Option<Self> {
//         path.into().ok().map(IncludeItem)
//     }
// }

// pub struct ExcludeList(pub Vec<PathBuf>);
// pub struct IncludeList(pub Vec<PathBuf>);

// impl ExcludeList {
//     pub fn new(base_dir: &Path, excludes: Vec<PathBuf>) -> Self {
//         let mut list = Vec::new();

//         // Add user-specified excludes
//         list.extend(excludes.into_iter().filter_map(|p| ExcludeItem::new(base_dir, p).map(|item| item.0)));

//         ExcludeList(list)
//     }

//     pub fn contains(&self, path: &Path) -> bool {
//         self.0.contains(&path.to_path_buf())
//     }
// }

// impl IncludeList {
//     pub fn new(base_dir: &Path, includes: Vec<PathBuf>) -> Self {
//         let list: Vec<PathBuf> = includes.into_iter()
//             .filter_map(|p| IncludeItem::new(base_dir, p).map(|item| item.0))
//             .collect();

//         IncludeList(list)
//     }

//     pub fn contains(&self, path: &Path) -> bool {
//         self.0.contains(&path.to_path_buf())
//     }
// }
