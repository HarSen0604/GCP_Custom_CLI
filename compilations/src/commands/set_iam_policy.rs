// commands/set_iam_policy.rs
use super::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let project = "my-default-project";
    let policy_file = args.get("policy_file").map(|s| s.as_str()).unwrap_or("policy.json");
    execute_command::execute_command("gcloud", &["projects", "set-iam-policy", project, policy_file]);
}