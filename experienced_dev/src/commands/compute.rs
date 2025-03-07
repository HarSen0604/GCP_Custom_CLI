use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_instances(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let zone = &args["ZONE"];
    println!("Listing VM instances in zone {}...", zone);
    run_command("gcloud", &["compute", "instances", "list", "--zones", zone], log_file);
}

pub fn create_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let instance_name = &args["INSTANCE_NAME"];
    let zone = &args["ZONE"];
    let machine_type = &args["MACHINE_TYPE"];
    println!("Creating VM instance '{}'...", instance_name);
    run_command(
        "gcloud",
        &[
            "compute",
            "instances",
            "create",
            instance_name,
            "--zone",
            zone,
            "--machine-type",
            machine_type,
        ],
        log_file
    );
}

pub fn delete_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let instance_name = &args["INSTANCE_NAME"];
    let zone = &args["ZONE"];
    println!("Deleting VM instance '{}'...", instance_name);
    run_command(
        "gcloud",
        &["compute", "instances", "delete", instance_name, "--zone", zone],
        log_file
    );
}

pub fn start_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let instance_name = &args["INSTANCE_NAME"];
    let zone = &args["ZONE"];
    println!("Starting VM instance '{}'...", instance_name);
    run_command(
        "gcloud",
        &["compute", "instances", "start", instance_name, "--zone", zone],
        log_file
    );
}

pub fn stop_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let instance_name = &args["INSTANCE_NAME"];
    let zone = &args["ZONE"];
    println!("Stopping VM instance '{}'...", instance_name);
    run_command(
        "gcloud",
        &["compute", "instances", "stop", instance_name, "--zone", zone],
        log_file
    );
}

pub fn ssh_to_instance(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let instance_name = &args["INSTANCE_NAME"];
    let zone = &args["ZONE"];
    println!("Connecting to VM instance '{}' via SSH...", instance_name);
    run_command("gcloud", &["compute", "ssh", instance_name, "--zone", zone], log_file);
}
