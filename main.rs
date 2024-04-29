cargo new graph_analysis
cd graph_analysis
[dependencies]
petgraph = "0.6.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

use petgraph::Graph;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut graph = Graph::<&str, &str>::new();
    // Assuming each line in your JSON file represents an edge in the graph.
    let data = load_data("/Users/itaimerlin/Downloads/facebook_large/musae_facebook_features.json").expect("Could not load data");
    for edge in data {
        let (source, target) = parse_edge(&edge);
        let source_id = graph.add_node(source);
        let target_id = graph.add_node(target);
        graph.add_edge(source_id, target_id, "interest");
    }
    fn parse_edge(edge: &Value) -> Result<(&str, &str), &'static str> {
        let source = edge["source"].as_str().ok_or("Missing source")?;
        let target = edge["target"].as_str().ok_or("Missing target")?;
        Ok((source, target))
    }
    
}

fn load_data(file_path: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: Vec<Value> = serde_json::from_str(&contents)?;
    Ok(data)
}

fn parse_edge(edge: &Value) -> Result<(&str, &str), &'static str> {
    let source = edge["source"].as_str().ok_or("Missing source")?;
    let target = edge["target"].as_str().ok_or("Missing target")?;
    Ok((source, target))
}


git init
git add .
git commit -m "Initial commit with crate setup"


git remote add origin < https://snap.stanford.edu/data/facebook_large.zip>
git branch -M main
git push -u origin main


