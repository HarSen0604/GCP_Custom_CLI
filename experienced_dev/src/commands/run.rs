use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

pub fn list_services(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let region = &args["REGION"];
    println!("Listing Cloud Run services in region {}...", region);
    run_command("gcloud", &["run", "services", "list", "--region", region], log_file);
}

pub fn deploy_service(args: &HashMap<String, String>, log_file: &mut BufWriter<std::fs::File>) {
    let service_name = &args["SERVICE_NAME"];
    let image_name = &args["IMAGE_NAME"];
    let region = &args["REGION"];
    println!("Deploying Cloud Run service '{}'...", service_name);
    run_command(
        "gcloud",
        &["run", "deploy", service_name, "--image", image_name, "--region", region],
        log_file
    );
}
