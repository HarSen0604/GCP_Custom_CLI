use inquire::{Select, Text};
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Virtual Machines & Compute Engine category...");

    let options = vec![
        "1. List Compute Engine zones (gcloud compute zones list)",
        "2. Describe a VM instance (gcloud compute instances describe <instance>)",
        "3. List all VM instances (gcloud compute instances list)",
        "4. Create a snapshot of a disk (gcloud compute disks snapshot <disk>)",
        "5. Describe a snapshot (gcloud compute snapshots describe <snapshot>)",
        "6. Delete a snapshot (gcloud compute snapshots delete <snapshot>)",
        "7. Connect to a VM via SSH (gcloud compute ssh <instance>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Virtual Machines & Compute Engine - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. List Compute Engine zones (gcloud compute zones list)" => {
                run_command("gcloud compute zones list");
            }
            "2. Describe a VM instance (gcloud compute instances describe <instance>)" => {
                let instance = Text::new("Enter VM instance name:").prompt().unwrap();
                run_command(&format!("gcloud compute instances describe {}", instance));
            }
            "3. List all VM instances (gcloud compute instances list)" => {
                run_command("gcloud compute instances list");
            }
            "4. Create a snapshot of a disk (gcloud compute disks snapshot <disk>)" => {
                let disk = Text::new("Enter disk name:").prompt().unwrap();
                run_command(&format!("gcloud compute disks snapshot {}", disk));
            }
            "5. Describe a snapshot (gcloud compute snapshots describe <snapshot>)" => {
                let snapshot = Text::new("Enter snapshot name:").prompt().unwrap();
                run_command(&format!("gcloud compute snapshots describe {}", snapshot));
            }
            "6. Delete a snapshot (gcloud compute snapshots delete <snapshot>)" => {
                let snapshot = Text::new("Enter snapshot name to delete:").prompt().unwrap();
                run_command(&format!("gcloud compute snapshots delete {}", snapshot));
            }
            "7. Connect to a VM via SSH (gcloud compute ssh <instance>)" => {
                let instance = Text::new("Enter VM instance name:").prompt().unwrap();
                run_command(&format!("gcloud compute ssh {}", instance));
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