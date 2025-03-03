use std::{fs::{DirEntry, Metadata}, path::PathBuf};

use chrono::{DateTime, Utc};

use crate::{extraction::NodeParseErrorKind, icon, statistics::statistics_from};

pub struct NodeStatistics {
    /* Metric that defines how much time has this file is not acessed */
    pub starvation:    f64,
    /* Metric that defines how many subnodes are stored inside current node */
    pub bloat:         f64,
    pub depth:         usize
}

#[warn(dead_code)]
pub struct Node {
    pub path:              PathBuf,
    pub name:              String,
    pub directory:         bool,
    pub metadata:          Metadata,
    pub statistics:        Option<NodeStatistics>
}

impl Node {
    pub fn from(entry: DirEntry) -> Result<Self, NodeParseErrorKind> {
        if let Err(_) = entry.file_name().into_string() {
            return Err(NodeParseErrorKind::CouldNotExtractFileName());
        }
        if let Err(_) = entry.metadata() {
            return Err(NodeParseErrorKind::CouldNotExtractMetadata(entry.file_name()))
        }
        if let Err(_) = entry.file_type() {
            return Err(NodeParseErrorKind::CouldNotExtractFileType(entry.file_name()))
        }
        return Ok(Node{
            path: entry.path(),
            name: entry.file_name().into_string().unwrap(),
            metadata: entry.metadata().unwrap(),
            directory: entry.file_type().unwrap().is_dir(),
            statistics: None
        });
    }

    pub fn calc_statistics(&mut self) {
        self.statistics = Some(statistics_from(&self));
    }

    pub fn info(&self) -> String {
        let mut formatted_date = None;
        let statistics_info = self.statistics_info();
        if let Ok(date) = self.metadata.accessed() {
            let datetime: DateTime<Utc> = date.into();
            formatted_date = Some(datetime.format("%d/%m/%Y %T").to_string())
        }
        let line = format!(
            "{} {} [{}] {}",
            icon::get_icon_by_node(self), 
            self.name,
            formatted_date.unwrap_or("ERRINDT!".to_string()),
            statistics_info
        );
        return line;
    }
    fn statistics_info(&self) -> String {
        if let Some(stats) = &self.statistics {
            return format!(
                "(sv: {:.2}, bt: {:.2} MB, d: {})",
                (stats.starvation * 100.0),
                stats.bloat / 1000_f64.powi(2),
                stats.depth
            );
        }
        return String::new();
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&self.info());
    }
}
