// commands/deploy_app.rs
use crate::commands::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let project = args.get("project").map(|s| s.as_str()).unwrap_or("my-default-project");
    let version = args.get("version").map(|s| s.as_str()).unwrap_or("v1");
    execute_command::execute_command("gcloud", &["config", "set", "project", project]);
    execute_command::execute_command("gcloud", &["app", "deploy", "--project", project, "--version", version]);
}