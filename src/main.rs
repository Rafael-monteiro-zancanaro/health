use std::{path::PathBuf, process::exit};
use nodes::{from_dir, Node};
use statistics::NodeStatistics;
mod icon;
mod nodes;
mod statistics;

fn main() {
    let relpath = check_args();
    
    if relpath.is_none() {
        usage();
        exit(1);
    }

    let canonical_path = canonize_rel_path(&relpath.unwrap());
    extract_from_path(canonical_path);
}

fn extract_from_path(path_buf: PathBuf) {
    if let Ok(dir) = std::fs::read_dir(&path_buf) {
        let nodes = from_dir(dir);
        nodes.iter().for_each(|node| println!("{}", NodeStatistics::from(node)));
    } else {
        eprintln!("Could not walk over directory {}", &path_buf.display());
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
    println!("\t health [RELPATH]||\"--help\"");
    println!("[RELPATH]: Relative path of folder that will be viewed");
    println!("--help: Shows detailed information about the program\n");
    println!("About the statistics");
    println!("sv (StarVation): The more closer to 100%, more unused the file/diretory is");
    println!("bt (BloaT): The more closer to 100%, more files it has inside of it");
}

