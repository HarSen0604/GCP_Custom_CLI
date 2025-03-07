use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_topics(log_file: &mut BufWriter<std::fs::File>) {
    println!("Listing Pub/Sub topics...");
    run_command("gcloud", &["pubsub", "topics", "list"], log_file);
}

pub fn create_topic(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let topic_name = &args["TOPIC_NAME"];
    println!("Creating Pub/Sub topic '{}'...", topic_name);
    run_command("gcloud", &["pubsub", "topics", "create", topic_name], log_file);
}
