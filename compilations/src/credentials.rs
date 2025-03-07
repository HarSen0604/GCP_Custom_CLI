use inquire::{Select, Text};
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Credentials category...");

    let options = vec![
        "1. Authorize Google Cloud access (gcloud auth login)",
        "2. Activate a service account (gcloud auth activate-service-account <email>)",
        "3. List all credentialed accounts (gcloud auth list)",
        "4. Display the current access token (gcloud auth print-access-token)",
        "5. Revoke access credentials (gcloud auth revoke <account>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Credentials - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Authorize Google Cloud access (gcloud auth login)" => {
                run_command("gcloud auth login");
            }
            "2. Activate a service account (gcloud auth activate-service-account <email>)" => {
                let email = Text::new("Enter service account email:").prompt().unwrap();
                run_command(&format!("gcloud auth activate-service-account {}", email));
            }
            "3. List all credentialed accounts (gcloud auth list)" => {
                run_command("gcloud auth list");
            }
            "4. Display the current access token (gcloud auth print-access-token)" => {
                run_command("gcloud auth print-access-token");
            }
            "5. Revoke access credentials (gcloud auth revoke <account>)" => {
                let account = Text::new("Enter the account email to revoke:").prompt().unwrap();
                run_command(&format!("gcloud auth revoke {}", account));
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
