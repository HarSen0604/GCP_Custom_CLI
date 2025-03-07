use inquire::{Select, Text};
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Projects category...");

    let options = vec![
        "1. Display project metadata (gcloud projects describe <project-id>)",
        "2. Add an IAM policy binding (gcloud projects add-iam-policy-binding <project-id> --member=<member> --role=<role>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Projects - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Display project metadata (gcloud projects describe <project-id>)" => {
                if let Ok(project_id) = Text::new("Enter the Project ID:").prompt() {
                    run_command(&format!("gcloud projects describe {}", project_id));
                }
            }
            "2. Add an IAM policy binding (gcloud projects add-iam-policy-binding <project-id> --member=<member> --role=<role>)" => {
                if let Ok(project_id) = Text::new("Enter the Project ID:").prompt() {
                    if let Ok(member) = Text::new("Enter the member (e.g., user:email@example.com):").prompt() {
                        if let Ok(role) = Text::new("Enter the IAM role (e.g., roles/editor):").prompt() {
                            run_command(&format!(
                                "gcloud projects add-iam-policy-binding {} --member={} --role={}",
                                project_id, member, role
                            ));
                        }
                    }
                }
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
