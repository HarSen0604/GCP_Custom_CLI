use inquire::{Select, Text};
use std::process::Command;

pub fn run() {
    println!("\nExecuting Personalization category...");

    let options = vec![
        "1. Set a configuration property (gcloud config set <property> <value>)",
        "2. Get value of a configuration property (gcloud config get <property>)",
        "3. List all configuration properties (gcloud config list)",
        "4. Create a new named configuration (gcloud config configurations create <name>)",
        "5. List all available configurations (gcloud config configurations list)",
        "6. Activate an existing named configuration (gcloud config configurations activate <name>)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Personalization - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Set a configuration property (gcloud config set <property> <value>)" => {
                if let Ok(property) = Text::new("Enter the property (e.g., compute/zone):").prompt() {
                    if let Ok(value) = Text::new("Enter the value:").prompt() {
                        run_command(&format!("gcloud config set {} {}", property, value));
                    }
                }
            }
            "2. Get value of a configuration property (gcloud config get <property>)" => {
                if let Ok(property) = Text::new("Enter the property:").prompt() {
                    run_command(&format!("gcloud config get {}", property));
                }
            }
            "3. List all configuration properties (gcloud config list)" => {
                run_command("gcloud config list");
            }
            "4. Create a new named configuration (gcloud config configurations create <name>)" => {
                if let Ok(name) = Text::new("Enter the configuration name:").prompt() {
                    run_command(&format!("gcloud config configurations create {}", name));
                }
            }
            "5. List all available configurations (gcloud config configurations list)" => {
                run_command("gcloud config configurations list");
            }
            "6. Activate an existing named configuration (gcloud config configurations activate <name>)" => {
                if let Ok(name) = Text::new("Enter the configuration name:").prompt() {
                    run_command(&format!("gcloud config configurations activate {}", name));
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
