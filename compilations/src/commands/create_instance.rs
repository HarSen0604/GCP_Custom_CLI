// commands/create_instance.rs
use super::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let instance = args.get("instance").map(|s| s.as_str()).unwrap_or("my-instance");
    let machine_type = args.get("machine_type").map(|s| s.as_str()).unwrap_or("n1-standard-1");
    execute_command::execute_command("gcloud", &["compute", "instances", "create", instance]);
    execute_command::execute_command("gcloud", &["compute", "instances", "set-machine-type", instance, "--machine-type", machine_type]);
    execute_command::execute_command("gcloud", &["compute", "instances", "add-tags", instance, "--tags=http-server"]);
}