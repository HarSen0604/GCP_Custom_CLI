// commands/create_cluster.rs
use super::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let cluster = args.get("cluster").map(|s| s.as_str()).unwrap_or("my-cluster");
    let num_nodes = args.get("num_nodes").map(|s| s.as_str()).unwrap_or("3");
    let min_nodes = args.get("min_nodes").map(|s| s.as_str()).unwrap_or("1");
    let max_nodes = args.get("max_nodes").map(|s| s.as_str()).unwrap_or("5");
    execute_command::execute_command("gcloud", &["container", "clusters", "create", cluster, "--num-nodes", num_nodes, "--enable-autoscaling", "--min-nodes", min_nodes, "--max-nodes", max_nodes]);
}