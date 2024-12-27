use std::{
    fs::{self},
    path::Path,
};

pub fn get_git_repositories(dir: &Path) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    if dir.is_dir() {
        if dir.join(".git/").exists() {
            output.push(dir.display().to_string());
            return output;
        }
        for result in dir.read_dir().expect("Could not read directory") {
            // for entry in result {
            let entry = result.expect("No result in directory");
            // if let Ok(entry) = result {
            let path = entry.path();
            if path.is_dir() {
                let subdirs = get_git_repositories(&path);
                for subdir in subdirs {
                    output.push(subdir);
                }
            }
            // }
        }
    }

    return output;
}
