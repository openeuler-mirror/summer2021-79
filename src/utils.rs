use std::path::PathBuf;
use walkdir;


pub fn iter_rs_fpath(path_str: &str) -> Vec<PathBuf> {
    let input_path: PathBuf = PathBuf::from(path_str);

    if input_path.is_file() {
        vec![input_path]
    } else {
        let rs_fpath_vec: Vec<PathBuf> = walkdir::WalkDir::new(input_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(
                |e| e.path().extension().map(|s| s == "rs").unwrap_or(false)
            )
            .map(walkdir::DirEntry::into_path)
            .collect();
        rs_fpath_vec
    }
}