// commands/remove_iam_policy.rs
use super::execute_command;
use std::collections::HashMap;

pub fn execute(args: &HashMap<String, String>) {
    let project = "my-default-project";
    let member = args.get("member").map(|s| s.as_str()).unwrap_or("user:example@example.com");
    let role = args.get("role").map(|s| s.as_str()).unwrap_or("roles/viewer");
    execute_command::execute_command("gcloud", &["projects", "remove-iam-policy-binding", project, "--member", member, "--role", role]);
}