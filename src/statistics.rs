use std::os::unix::fs::MetadataExt;

use chrono::{DateTime, Duration, Utc};

use crate::{extraction, node::{Node, NodeStatistics}};

pub fn statistics_from(node: &Node) -> NodeStatistics {
    let bloat = calc_bloat(node);
    let datetime: DateTime<Utc> = node.metadata.clone().modified().unwrap().into();
    let starvation = calc_starvation(datetime);
    return NodeStatistics {
        starvation: starvation,
        bloat: bloat,
        depth: 0
    };
}

fn calc_bloat(node: &Node) -> f64 {
    if !node.directory {
        return 1_f64;
    }
    let node_path = node.path.clone();
    if let Ok(dir) = std::fs::read_dir(node_path) {
        let subnodes = extraction::run_over_directory(dir);
        let subnodes_bytesize = subnodes.iter().map(|sub_node| {
            sub_node.metadata.size()
        }).reduce(|a, b| a+b).unwrap_or(0_u64);
        return subnodes_bytesize as f64 / subnodes.len() as f64;
    }
    return 0.0;
}

fn calc_starvation(modified_date: DateTime<Utc>) -> f64 {
    let current_date = Utc::now();
    let time_diff = current_date - modified_date;
    let max_period = Duration::days(365);
    if time_diff > max_period {
        1.0
    } else {
        time_diff.num_seconds() as f64 / max_period.num_seconds() as f64
    }
}