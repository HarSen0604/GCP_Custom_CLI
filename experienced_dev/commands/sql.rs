use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn create_sql_instance(
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    let sql_instance = &args["SQL_INSTANCE"];
    let region = &args["REGION"];
    println!("Creating Cloud SQL instance '{}'...", sql_instance);
    run_command(
        "gcloud",
        &["sql", "instances", "create", sql_instance, "--region", region],
        log_file
    );
}

pub fn start_sql_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let sql_instance = &args["SQL_INSTANCE"];
    println!("Starting Cloud SQL instance '{}'...", sql_instance);
    run_command(
        "gcloud",
        &["sql", "instances", "patch", sql_instance, "--activation-policy", "ALWAYS"],
        log_file
    );
}

pub fn stop_sql_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let sql_instance = &args["SQL_INSTANCE"];
    println!("Stopping Cloud SQL instance '{}'...", sql_instance);
    run_command(
        "gcloud",
        &["sql", "instances", "patch", sql_instance, "--activation-policy", "NEVER"],
        log_file
    );
}
