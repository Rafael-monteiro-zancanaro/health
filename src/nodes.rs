use std::{
        ffi::OsString,
        fs::{DirEntry, Metadata, ReadDir},
        path::PathBuf,
};

pub enum NodeParseErrorKind {
        CouldNotExtractFileName(),
        CouldNotExtractMetadata(OsString),
        CouldNotExtractFileType(OsString),
}

#[derive(Clone)]
pub struct Node {
        pub path:       PathBuf,
        pub name:       String,
        pub directory:  bool,
        pub metadata:   Metadata,
}

impl Node 
{
        pub fn from(entry: DirEntry) -> Result<Self, NodeParseErrorKind> {
                if let Err(_) = entry.file_name().into_string() {
                        return Err(NodeParseErrorKind::CouldNotExtractFileName());
                }
                if let Err(_) = entry.metadata() {
                        return Err(NodeParseErrorKind::CouldNotExtractMetadata(
                                entry.file_name(),
                        ));
                }
                if let Err(_) = entry.file_type() {
                        return Err(NodeParseErrorKind::CouldNotExtractFileType(
                                entry.file_name(),
                        ));
                }
                return Ok(Node {
                        path: entry.path(),
                        name: entry.file_name().into_string().unwrap(),
                        metadata: entry.metadata().unwrap(),
                        directory: entry.file_type().unwrap().is_dir(),
                });
        }
}

pub fn from_path(path_buf: PathBuf) -> Result<Vec<Node>, String> 
{
        if let Ok(dir) = std::fs::read_dir(&path_buf) {
                return Ok(from_dir(dir));
        }
        return Err(format!(
                "Could not walk over directory {}",
                &path_buf.display()
        ));
}

pub fn from_dir(dir: ReadDir) -> Vec<Node> 
{
        let mut nodes: Vec<Node> = Vec::new();
        for opt_entry in dir {
                if let Err(_) = opt_entry {continue;}
                let entry = opt_entry.unwrap();
                
                match Node::from(entry) {

                        Ok(node) => nodes.push(node),
                        Err(extraction_error) => {
                                show_extraction_error_reason(extraction_error)
                        },

                }
        }
        return nodes;
}

fn show_extraction_error_reason(extraction_error: NodeParseErrorKind) {
        return match extraction_error {

                NodeParseErrorKind::CouldNotExtractFileName() => {
                        eprintln!("Could not get file name from file")
                }
                NodeParseErrorKind::CouldNotExtractFileType(filename) => {
                        eprintln!("Could not get file type from {:?}", filename)
                }
                NodeParseErrorKind::CouldNotExtractMetadata(filename) => {
                        eprintln!("Could not get metadata from {:?}", filename)
                }

        };
}
