use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_datasets(log_file: &mut BufWriter<std::fs::File>) {
    println!("Listing BigQuery datasets...");
    run_command("bq", &["ls"], log_file);
}

pub fn run_query(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    if args.contains_key("QUERY_STRING") {
        let query = &args["QUERY_STRING"];
        println!("Running BigQuery SQL query...");
        run_command("bq", &["query", "--nouse_legacy_sql", query], log_file);
    } else {
        println!("❌ Please provide a query using --QUERY_STRING.");
    }
}

pub fn create_table(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let dataset_name = &args["DATASET_NAME"];
    let table_name = &args["TABLE_NAME"];
    println!("Creating BigQuery table '{}.{}'...", dataset_name, table_name);
    run_command("bq", &["mk", "--table", &format!("{}:{}", dataset_name, table_name)], log_file);
}
