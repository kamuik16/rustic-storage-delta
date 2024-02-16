use std::{env, fs, str, path::Path, process::Command};
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
        let _repo = match Repository::clone(&args[1], &cache_path) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone: {}", e),
        };

        // Run forge install command
        if Command::new("forge")
        .current_dir(&cache_path)
        .arg("install")
        .status()
        .expect("Failed to run forge install!")
        .success() 
        {
            println!("forge install successful!");
        } else 
        {
            println!("forge install failed!");
        }
    }

    // Declare vectors to store the .sol file names
    let files_with_path_old: Vec<String>;
    let files_with_path_new: Vec<String>;

    // Find .sol files in the old version directory
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

    // Find .sol files in the new version directory
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
    match fs::remove_file("rustic-storage-delta-main/.removed") {
        Ok(_) => println!(".removed file deleted successfully!"),
        Err(_) => (),
    }

    let mut deleted_files: Vec<String> = vec![];
    let mut deleted_content = String::new();

    for file_path in &files_with_path_old {
        // Check for deleted files
        if !files_with_path_new.contains(file_path) {
            deleted_files.push(file_path.to_string());
            deleted_content.push_str(file_path);
            deleted_content.push('\n');
        }
    }
    // Write deleted file names to .removed file
    match fs::write("rustic-storage-delta-main/.removed", &deleted_content) {
        Ok(_) => (),
        Err(err) => println!("Error writing to .removed file: {}", err),
    }

    for file in &files_with_path_old {
        let contract_name = Path::new(file).file_stem().unwrap().to_str().unwrap();

        // Run the 'forge inspect' command in the old directory
        let output_old = Command::new("forge")
            .current_dir("rustic-storage-delta-cache/src/")
            .args(["inspect", &contract_name, "storage"])
            .output()
            .expect("Failed to run forge inspect!");

        // Run the 'forge inspect' command in the new directory
        let output_new = Command::new("forge")
            .current_dir("src/")
            .args(["inspect", &contract_name, "storage"])
            .output()
            .expect("Failed to run forge inspect!");

        // Run the node script
        let node_script_status = Command::new("node")
            .arg("./lib/rustic-storage-delta/_reporter.js")
            .arg(str::from_utf8(&output_old.stdout).unwrap())
            .arg(str::from_utf8(&output_new.stdout).unwrap())
            .arg(file)
            .arg("0")
            .status()
            .expect("Failed to execute node script!");

        // Check the status of the Node.js script
        if node_script_status.success() {
            println!("Node script ran successfully!");
        } else {
            let node_script_status_err = format!("Node script failed: {}", node_script_status);
            println!("{}", node_script_status_err);
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