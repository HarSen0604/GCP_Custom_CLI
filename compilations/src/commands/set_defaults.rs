// commands/set_defaults.rs
use super::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let project = args.get("project").map(|s| s.as_str()).unwrap_or("my-default-project");
    let region = args.get("region").map(|s| s.as_str()).unwrap_or("us-central1");
    let zone = args.get("zone").map(|s| s.as_str()).unwrap_or("us-central1-a");
    execute_command::execute_command("gcloud", &["config", "set", "project", project]);
    execute_command::execute_command("gcloud", &["config", "set", "compute/region", region]);
    execute_command::execute_command("gcloud", &["config", "set", "compute/zone", zone]);
}