// commands/create_network.rs
use super::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let network = args.get("network").map(|s| s.as_str()).unwrap_or("my-network");
    let firewall_rule = args.get("firewall_rule").map(|s| s.as_str()).unwrap_or("allow-http");
    execute_command::execute_command("gcloud", &["compute", "networks", "create", network, "--subnet-mode=custom"]);
    execute_command::execute_command("gcloud", &["compute", "firewall-rules", "create", firewall_rule, "--network", network, "--allow=tcp:80"]);
}