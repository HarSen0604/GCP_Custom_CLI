use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_buckets(log_file: &mut BufWriter<std::fs::File>) {
    println!("Listing Cloud Storage buckets...");
    run_command("gcloud", &["storage", "buckets", "list"], log_file);
}

pub fn create_bucket(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let bucket_name = &args["BUCKET_NAME"];
    println!("Creating Cloud Storage bucket '{}'...", bucket_name);
    run_command(
        "gcloud",
        &["storage", "buckets", "create", &format!("gs://{}", bucket_name)],
        log_file
    );
}

pub fn delete_bucket(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let bucket_name = &args["BUCKET_NAME"];
    println!("Deleting Cloud Storage bucket '{}'...", bucket_name);
    run_command(
        "gcloud",
        &["storage", "rm", "--recursive", &format!("gs://{}", bucket_name)],
        log_file
    );
}
