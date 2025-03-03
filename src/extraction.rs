use std::{ffi::OsString, fs::ReadDir};

use crate::{config, node::Node};

pub enum NodeParseErrorKind {
    CouldNotExtractFileName(),
    CouldNotExtractMetadata(OsString),
    CouldNotExtractFileType(OsString)
}

pub fn run_over_directory(dir: ReadDir) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    for opt_entry in dir {
        if let Err(_) = opt_entry {
            continue;
        }
        let entry = opt_entry.unwrap();
        match Node::from(entry) {
            Ok(mut node) => {
                if config::SHOW_STATISTICS {
                    node.calc_statistics();
                }
                nodes.push(node)
            } ,
            Err(extraction_error) => show_extraction_error_reason(extraction_error)
        }
    }
    return nodes;
}


fn show_extraction_error_reason(extraction_error: NodeParseErrorKind) {
    return match extraction_error {
        NodeParseErrorKind::CouldNotExtractFileName() => {
            eprintln!("Could not get file name from file")
        },
        NodeParseErrorKind::CouldNotExtractFileType(filename) => {
            eprintln!("Could not get file type from {:?}", filename)
        },
        NodeParseErrorKind::CouldNotExtractMetadata(filename) => {
            eprintln!("Could not get metadata from {:?}", filename)
        }
    }

}