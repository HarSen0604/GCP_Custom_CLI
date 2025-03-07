use inquire::{Select, Text};
use std::process::Command;

pub fn run() {
    println!("\nExecuting Miscellaneous category...");

    let options = vec![
        "1. Decrypt ciphertext using Cloud KMS key (gcloud kms decrypt)",
        "2. List your project's logs (gcloud logging logs list)",
        "3. Display Cloud SQL instance backup info (gcloud sql backups describe <backup-id>)",
        "4. Export data from Cloud SQL instance to a SQL file (gcloud sql export sql <instance> <gcs-path>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Miscellaneous - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Decrypt ciphertext using Cloud KMS key (gcloud kms decrypt)" => {
                let key = Text::new("Enter the KMS key name:").prompt().unwrap();
                let input_file = Text::new("Enter the ciphertext file path:").prompt().unwrap();
                let output_file = Text::new("Enter the plaintext output file path:").prompt().unwrap();
                run_command(&format!(
                    "gcloud kms decrypt --key={} --ciphertext-file={} --plaintext-file={}",
                    key, input_file, output_file
                ));
            }
            "2. List your project's logs (gcloud logging logs list)" => {
                run_command("gcloud logging logs list");
            }
            "3. Display Cloud SQL instance backup info (gcloud sql backups describe <backup-id>)" => {
                let backup_id = Text::new("Enter the backup ID:").prompt().unwrap();
                run_command(&format!("gcloud sql backups describe {}", backup_id));
            }
            "4. Export data from Cloud SQL instance to a SQL file (gcloud sql export sql <instance> <gcs-path>)" => {
                let instance = Text::new("Enter the Cloud SQL instance name:").prompt().unwrap();
                let gcs_path = Text::new("Enter the Google Cloud Storage path:").prompt().unwrap();
                run_command(&format!("gcloud sql export sql {} {}", instance, gcs_path));
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
