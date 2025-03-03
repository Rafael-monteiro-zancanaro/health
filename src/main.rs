use std::{path::PathBuf, process::exit};
use node::Node;
mod icon;
mod node;
mod statistics;
mod config;
mod extraction;

fn main() {
    let relpath = check_args();
    
    if relpath.is_none() {
        usage();
        exit(1);
    }

    let canonical_path = canonize_rel_path(&relpath.unwrap());

    if let Ok(dir) = std::fs::read_dir(&canonical_path) {
        let nodes = extraction::run_over_directory(dir);
        nodes.iter().for_each(|node| {
            println!("{}", node)
        });
    } else {
        eprintln!("Could not walk over directory {}", &canonical_path.display());
    }
}

fn canonize_rel_path(relpath: &str) -> PathBuf {
    let opt_canonical_path = std::fs::canonicalize(relpath.to_string());
    if opt_canonical_path.is_err() {
        eprintln!("Cannot open relative path {}", relpath.to_string());
        panic!();
    }
    return opt_canonical_path.unwrap();
}

fn check_args() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return None;
    }

    return Some(args[1].to_string());
}

fn usage() {
    println!("USAGE:");
    println!("\t health [RELPATH]");
    println!("RELPATH: Relative path of folder that will be viewed");
}

