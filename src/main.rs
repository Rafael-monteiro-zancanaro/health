use std::{path::PathBuf, process::exit};
use nodes::Node;
use statistics::NodeStatistics;
mod icon;
mod nodes;
mod statistics;

/* Represents a fatal failure on program. Looks like "EXIT_FAILURE" from <stdlib.h> */
const PROCESS_FAILURE: i32 = 1;

/* Program entrypoint */
fn main() {
    let relpath = args_info();
    
    if relpath.is_none() {
        usage();
        exit(PROCESS_FAILURE);
    }

    let canonical_path = abs_path_of(&relpath.unwrap());

    let nodes = nodes::from_path(canonical_path);

    if let Err(checking_error) = nodes {
        eprintln!("Error: {}", checking_error);
        exit(PROCESS_FAILURE);
    }

    nodes.unwrap()
        .iter()
        .for_each(|node| println!("{}", NodeStatistics::from(node)));
}

/*
    * abs_path_of: Converts a relative path to a absolute_path

    This function can fail, and if so, the program will exit with a PROCESS_FAIL output.

    * @relpath: The string that represents the relative path from command line arguments.
    * Output: A PathBuf structure with the absolute path that represents the relative path given.
*/
fn abs_path_of(relpath: &str) -> PathBuf {
    let opt_canonical_path = std::fs::canonicalize(relpath.to_string());
    if opt_canonical_path.is_err() {
        eprintln!("Cannot open relative path {}", relpath.to_string());
        exit(PROCESS_FAILURE);
    }
    return opt_canonical_path.unwrap();
}

/*
    * args_info: Get the commands given on command line arguments

    * This function is pretty much simple, and needs to be reinvented 
    as a function returning a struct or an algebraic type.

    * Output: An Option<> enum that carries the relative path given on command line arguments.  
*/
fn args_info() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return None;
    }

    return Some(args[1].to_string());
}

/*
    Usage: Prints the usage of the program.

    It is called when someone does not know what is doing.
*/
fn usage() {
    println!("USAGE:");
    println!("\t health [RELPATH]||\"--help\"");
    println!("[RELPATH]: Relative path of folder that will be viewed");
    println!("--help: Shows detailed information about the program\n");
    println!("About the statistics");
    println!("sv (StarVation): The more closer to 100%, more unused the file/diretory is");
    println!("bt (BloaT): The more closer to 100%, more files it has inside of it");
}

