use inquire::{Select, Text};
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Help category...");

    let options = vec![
        "1. Search the gcloud tool reference documents (gcloud help)",
        "2. Provide feedback to the Google Cloud CLI team (gcloud feedback)",
        "3. View supplementary help topics (gcloud topic <topic>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Help - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Search the gcloud tool reference documents (gcloud help)" => {
                run_command("gcloud help");
            }
            "2. Provide feedback to the Google Cloud CLI team (gcloud feedback)" => {
                run_command("gcloud feedback");
            }
            "3. View supplementary help topics (gcloud topic <topic>)" => {
                if let Ok(topic) = Text::new("Enter topic name:").prompt() {
                    if !topic.is_empty() {
                        run_command(&format!("gcloud topic {}", topic));
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