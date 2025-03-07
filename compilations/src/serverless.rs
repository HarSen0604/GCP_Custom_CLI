use inquire::Select;
use std::process::Command;
use colored::*;

pub fn run() {
    println!("\nExecuting Serverless & App Engine category...");

    let options = vec![
        "1. Deploy app to App Engine (gcloud app deploy)",
        "2. List all versions of deployed services (gcloud app versions list)",
        "3. Open the app in a browser (gcloud app browse)",
        "4. Create an App Engine app (gcloud app create)",
        "5. Read the latest app logs (gcloud app logs read)",
        "0. Back",
    ];

    loop {
        let choice = Select::new("Serverless & App Engine - Select an option:", options.clone())
            .prompt()
            .unwrap_or("0. Back");

        match choice {
            "1. Deploy app to App Engine (gcloud app deploy)" => {
                run_command("gcloud app deploy");
            }
            "2. List all versions of deployed services (gcloud app versions list)" => {
                run_command("gcloud app versions list");
            }
            "3. Open the app in a browser (gcloud app browse)" => {
                run_command("gcloud app browse");
            }
            "4. Create an App Engine app (gcloud app create)" => {
                run_command("gcloud app create");
            }
            "5. Read the latest app logs (gcloud app logs read)" => {
                run_command("gcloud app logs read");
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
