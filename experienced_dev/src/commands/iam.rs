use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_iam_policy(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let project_id = &args["PROJECT_ID"];
    println!("Listing IAM policies for project '{}'...", project_id);
    run_command("gcloud", &["projects", "get-iam-policy", project_id], log_file);
}

pub fn add_iam_binding(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    if args.contains_key("MEMBER") && args.contains_key("ROLE") {
        let project_id = &args["PROJECT_ID"];
        let member = &args["MEMBER"];
        let role = &args["ROLE"];
        println!("Adding IAM role binding for '{}' with role '{}'...", member, role);
        run_command(
            "gcloud",
            &["projects", "add-iam-policy-binding", project_id, "--member", member, "--role", role],
            log_file
        );
    } else {
        println!("❌ Please provide both --MEMBER and --ROLE arguments.");
    }
}
