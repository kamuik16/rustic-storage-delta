use std::env;
use std::fs::read_dir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: lib/rustic-storage-delta/target/debug/rustic-storage-delta <commit_hash>");
        return;
    }

    let commit_hash: &String = &args[1];
    println!("Commit hash: {commit_hash}");

    match find_sol_files_recursive("src") {
        Ok(files) => {
            for file in files {
                println!("{}", file);
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn find_sol_files_recursive(dir: &str) -> Result<Vec<String>, std::io::Error> {
    let mut sol_files: Vec<String> = Vec::new();

    for entry in read_dir(dir)? {
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