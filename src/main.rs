use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: target/debug/rustic-storage-delta <commit_hash>");
        return;
    }

    let commit_hash: &String = &args[1];
    println!("Commit hash: {commit_hash}");
}
