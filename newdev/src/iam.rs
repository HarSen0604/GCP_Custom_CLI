use inquire::{Select, Text};
use std::process::Command;

pub fn run() {
    println!("\nExecuting IAM category...");

    let options = vec![
        "1. List IAM grantable roles (gcloud iam list-grantable-roles <resource>)",
        "2. Create a custom role (gcloud iam roles create <role-name>)",
        "3. Create a service account (gcloud iam service-accounts create <account-name>)",
        "4. Add IAM policy binding to service account (gcloud iam service-accounts add-iam-policy-binding <account>)",
        "5. Replace IAM policy binding (gcloud iam service-accounts set-iam-policy <account>)",
        "6. List service account keys (gcloud iam service-accounts keys list <account>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("IAM - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. List IAM grantable roles (gcloud iam list-grantable-roles <resource>)" => {
                if let Ok(resource) = Text::new("Enter resource URL:").prompt() {
                    run_command(&format!("gcloud iam list-grantable-roles --resource={}", resource));
                }
            }
            "2. Create a custom role (gcloud iam roles create <role-name>)" => {
                if let Ok(role_name) = Text::new("Enter role name:").prompt() {
                    run_command(&format!("gcloud iam roles create {}", role_name));
                }
            }
            "3. Create a service account (gcloud iam service-accounts create <account-name>)" => {
                if let Ok(account_name) = Text::new("Enter service account name:").prompt() {
                    run_command(&format!("gcloud iam service-accounts create {}", account_name));
                }
            }
            "4. Add IAM policy binding to service account (gcloud iam service-accounts add-iam-policy-binding <account>)" => {
                if let Ok(account) = Text::new("Enter service account email:").prompt() {
                    run_command(&format!("gcloud iam service-accounts add-iam-policy-binding {}", account));
                }
            }
            "5. Replace IAM policy binding (gcloud iam service-accounts set-iam-policy <account>)" => {
                if let Ok(account) = Text::new("Enter service account email:").prompt() {
                    run_command(&format!("gcloud iam service-accounts set-iam-policy {}", account));
                }
            }
            "6. List service account keys (gcloud iam service-accounts keys list <account>)" => {
                if let Ok(account) = Text::new("Enter service account email:").prompt() {
                    run_command(&format!("gcloud iam service-accounts keys list --iam-account={}", account));
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
    println!("\nExecuting: {}\n", command);
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Error: Command exited with status {:?}", status);
    }
}
