use inquire::{Select, Text};
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Getting Started category...");

    let options = vec![
        "1. Initialize, authorize, and configure the gcloud tool (gcloud init)",
        "2. Display version and installed components (gcloud version)",
        "3. Install specific components (gcloud components install <component>)",
        "4. Update Google Cloud CLI to the latest version (gcloud components update)",
        "5. Set a default Google Cloud project (gcloud config set project <project-id>)",
        "6. Display current gcloud environment details (gcloud info)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Getting Started - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Initialize, authorize, and configure the gcloud tool (gcloud init)" => {
                run_command("gcloud init");
            }
            "2. Display version and installed components (gcloud version)" => {
                run_command("gcloud version");
            }
            "3. Install specific components (gcloud components install <component>)" => {
                if let Ok(component) = Text::new("Enter the component name to install:").prompt() {
                    run_command(&format!("gcloud components install {}", component));
                }
            }
            "4. Update Google Cloud CLI to the latest version (gcloud components update)" => {
                run_command("gcloud components update");
            }
            "5. Set a default Google Cloud project (gcloud config set project <project-id>)" => {
                if let Ok(project_id) = Text::new("Enter your Google Cloud project ID:").prompt() {
                    run_command(&format!("gcloud config set project {}", project_id));
                }
            }
            "6. Display current gcloud environment details (gcloud info)" => {
                run_command("gcloud info");
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