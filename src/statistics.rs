use std::os::unix::fs::MetadataExt;

use chrono::{
        DateTime, 
        Duration, 
        Utc
};

use crate::{
        icon::get_icon_by_node, 
        nodes::{
                from_dir, 
                Node
        }
};


pub struct NodeStatistics 
{
        /* Metric that defines how much time has this file is not acessed */
        pub starvation:    f64,
        /* Metric that defines how many subnodes are stored inside current node */
        pub bloat:         f64,
        pub depth:         usize,
        pub node:          Node
}

impl NodeStatistics 
{
        pub fn from(node: &Node) -> Self 
        {
                let datetime: DateTime<Utc> = node.metadata.clone()
                        .modified()
                        .unwrap()
                        .into();
                let starvation = calc_starvation(datetime);
                let bloat = calc_bloat(node);
                return Self {
                        starvation: starvation,
                        bloat: bloat,
                        depth: 0,
                        node: node.clone()
                };
        }

        fn node_info(&self) -> String 
        {
                let mut formatted_date = None;
                if let Ok(date) = self.node.metadata.accessed() {
                        let datetime: DateTime<Utc> = date.into();
                        formatted_date = Some(datetime.format(
                                "%d/%m/%Y %T"
                        ).to_string())
                }
                return format!(
                        "{} {} [{}] (st: {:.2}%, bt: {:.2} MB, d: {})",
                        get_icon_by_node(&self.node),
                        self.node.name,
                        formatted_date.unwrap_or(
                                "!ERINDT".to_string()
                        ),
                        self.starvation * 100.0,
                        self.bloat / 1024.0_f64.powi(2),
                        self.depth
                )
        }
}

impl std::fmt::Display for NodeStatistics 
{

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
        {
                return f.write_str(&self.node_info());
        }
}

fn calc_bloat(node: &Node) -> f64 
{
        if !node.directory {
                return 1_f64;
        }
        let node_path = node.path.clone();
        if let Ok(dir) = std::fs::read_dir(node_path) {
                let subnodes = from_dir(dir);
                let subnodes_bytesize = subnodes.iter()
                        .map(|sub_node| sub_node.metadata.size())
                        .reduce(|a, b| a+b)
                        .unwrap_or(0_u64);
                return subnodes_bytesize as f64 / subnodes.len() as f64;
        }

        return 0.0;
}

fn calc_starvation(modified_date: DateTime<Utc>) -> f64 
{
        let current_date = Utc::now();
        let time_diff = current_date - modified_date;
        let max_period = Duration::days(365);
        if time_diff > max_period {return 1.0}
        return time_diff.num_seconds() as f64 / max_period.num_seconds() as f64
        
}