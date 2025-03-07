use inquire::{Select, Text};
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Docker & GKE category...");

    let options = vec![
        "1. Register gcloud as Docker credential helper (gcloud auth configure-docker)",
        "2. Create a GKE cluster (gcloud container clusters create <name>)",
        "3. List GKE clusters (gcloud container clusters list)",
        "4. Get GKE cluster credentials (gcloud container clusters get-credentials <name>)",
        "5. List container image tags (gcloud container images list-tags <image-name>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Docker & GKE - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Register gcloud as Docker credential helper (gcloud auth configure-docker)" => {
                run_command("gcloud auth configure-docker");
            }
            "2. Create a GKE cluster (gcloud container clusters create <name>)" => {
                let cluster_name = Text::new("Enter cluster name:").prompt().unwrap();
                run_command(&format!("gcloud container clusters create {}", cluster_name));
            }
            "3. List GKE clusters (gcloud container clusters list)" => {
                run_command("gcloud container clusters list");
            }
            "4. Get GKE cluster credentials (gcloud container clusters get-credentials <name>)" => {
                let cluster_name = Text::new("Enter cluster name:").prompt().unwrap();
                run_command(&format!("gcloud container clusters get-credentials {}", cluster_name));
            }
            "5. List container image tags (gcloud container images list-tags <image-name>)" => {
                let image_name = Text::new("Enter container image name:").prompt().unwrap();
                run_command(&format!("gcloud container images list-tags {}", image_name));
            }
            "0. Back" => {
                println!("Returning to main menu...");
                break;
            }
            _ => println!("Invalid selection, please try again."),
        }
    }
}

fn run_command(command: &str) {
    println!("{}", "Executing...".yellow());
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("{}", format!("Error: Command exited with status {:?}", status).red());
    }
    else {
        println!("{}", format!("✅ Successfully executed").green());
    }
}
