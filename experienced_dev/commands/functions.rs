use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn deploy_function(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let function_name = &args["FUNCTION_NAME"];
    let runtime = &args["RUNTIME"];
    println!("Deploying Cloud Function '{}'...", function_name);
    run_command("gcloud", &["functions", "deploy", function_name, "--runtime", runtime], log_file);
}
