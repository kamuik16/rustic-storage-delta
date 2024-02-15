use std::{env, fs};
use git2::Repository;

fn main() {
    // Check if the repository URL argument is provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lib/rustic-storage-delta/target/debug/rustic-storage-delta <repo_url>");
        return;
    }

    // Define paths for the main and cache directories
    let main_path = "rustic-storage-delta-main";
    let cache_path = "rustic-storage-delta-cache";

    // Check if the cache directory already exists
    if fs::metadata(&cache_path).is_ok() {
        println!("Cache directory already exists!");
    } else {
        // Clone the repository if the cache directory doesn't exist
        let _repo = match Repository::clone(&args[1], cache_path) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone: {}", e),
        };
    }

    // Declare vectors to store the .sol file names
    let files_with_path_old: Vec<String>;
    let files_with_path_new: Vec<String>;

    // Call the function to find .sol files in the old version directory
    match find_sol_files_recursive("rustic-storage-delta-cache/src") {
        Ok(files) => {
            files_with_path_old = files;
            println!("Old .sol files: {:?}", files_with_path_old);
        },
        Err(err) => {
            println!("Error finding old .sol files: {}", err);
            return;
        }
    }

    // Call the function to find .sol files in the new version directory
    match find_sol_files_recursive("src") {
        Ok(files) => {
            files_with_path_new = files;
            println!("New .sol files: {:?}", files_with_path_new);
        },
        Err(err) => {
            println!("Error finding new .sol files: {}", err);
            return;
        }
    }

    // Check if the main directory already exists
    if fs::metadata(&main_path).is_ok() {
        println!("Main directory already exists!");
    } else {
        // Create the main directory if it doesn't exist
        match fs::create_dir_all(&main_path) {
            Ok(_) => println!("Created main directory!"),
            Err(err) => println!("Error creating main directory: {}", err),
        }
    }

    // REPORT DELETED FILES

    // Check and delete the .removed file if it already exists
    match fs::metadata("rustic-storage-delta-main/.removed") {
        Ok(_) => {
            fs::remove_file("rustic-storage-delta-main/.removed")
            .expect("Failed to delete .removed file");
        },
        Err(_) => (),
    }

    let mut deleted_files: Vec<String> = vec![];
    for file_path in &files_with_path_old {
        // Check for deleted files
        if !files_with_path_new.contains(file_path) {
            deleted_files.push(file_path.to_string());
            // Write deleted file names to a file
            match fs::write("rustic-storage-delta-main/.removed", file_path.to_string() + "\n") {
                Ok(_) => println!("Deleted file: {}", file_path),
                Err(err) => println!("Error writing to .removed file: {}", err),
            }
        }
    }
}

// Define a function to find .sol files recursively
fn find_sol_files_recursive(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut sol_files: Vec<String> = Vec::new();

    // Traverse the directory recursively
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if it's a directory
        if path.is_dir() {
            // Recursively call the function for subdirectories
            let sub_files = find_sol_files_recursive(path.to_str().unwrap())?;
            sol_files.extend(sub_files);
        } else if path.is_file() && path.extension().unwrap_or_default() == "sol" {
            // Add .sol file name to the vector
            sol_files.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    Ok(sol_files)
}