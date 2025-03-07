use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_clusters(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let zone = &args["ZONE"];
    println!("Listing GKE clusters in zone {}...", zone);
    run_command("gcloud", &["container", "clusters", "list", "--zone", zone], log_file);
}

pub fn create_cluster(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let cluster_name = &args["CLUSTER_NAME"];
    let zone = &args["ZONE"];
    println!("Creating GKE cluster '{}'...", cluster_name);
    run_command(
        "gcloud",
        &["container", "clusters", "create", cluster_name, "--zone", zone],
        log_file
    );
}
