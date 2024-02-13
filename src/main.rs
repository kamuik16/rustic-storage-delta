use std::env;
use std::fs;
use git2::Repository;

fn main() {

    // Check if the repository url argument is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lib/rustic-storage-delta/target/debug/rustic-storage-delta <repo_url>");
        return;
    }

    // CLONE OLD VERSION
    let repo_url: &String = &args[1];
    let cache_path = "rustic-storage-delta-cache";
    if fs::metadata(&cache_path).is_ok() {
        println!("rustic-storage-delta-cache already exists!");
    } else {
        let _repo = match Repository::clone(repo_url, cache_path) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone: {}", e),
        };
    }

    // Declare empty vectors to store the .sol file paths
    let files_with_path_old: Vec<String>;
    let files_with_path_new: Vec<String>;

    // Call the function for the old version directory
    match find_sol_files_recursive("rustic-storage-delta-cache/src") {
        Ok(files) => {
            files_with_path_old = files;
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    }

    // Call the function for the new version directory
    match find_sol_files_recursive("src") {
        Ok(files) => {
            files_with_path_new = files;
        },
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    }

    // REPORT DELETED ONES
    let mut differences: Vec<String> = vec![];
    for file_path in &files_with_path_old {
        if !files_with_path_new.contains(&file_path) {
            differences.push(file_path.to_string());
            match fs::write("rustic-storage-delta-cache/.removed", file_path.to_string()) {
                Ok(_) => println!("Uh-oh! Looks like some files are missing!"),
                Err(err) => println!("Error writing to file: {}", err),
            }
        }

    }
}

// ========================================================================

// Define a function to find .sol files
fn find_sol_files_recursive(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut sol_files: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = find_sol_files_recursive(path.to_str().unwrap())?;
            sol_files.extend(sub_files);
        } else if path.is_file() && path.extension().map_or(false, |ext| ext.to_ascii_lowercase() == "sol") {
            sol_files.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(sol_files)
}