use std::process::exit;
use inquire::Select;

mod getting_started;
mod help;
mod personalization;
mod credentials;
mod projects;
mod iam;
mod docker_gke;
mod vm_compute;
mod serverless;
mod miscellaneous;

fn main() {
    let categories = vec![
        "1. Getting Started",
        "2. Help",
        "3. Personalization",
        "4. Credentials",
        "5. Projects",
        "6. IAM",
        "7. Docker & GKE",
        "8. VM & Compute Engine",
        "9. Serverless & App Engine",
        "10. Miscellaneous",
        "0. Exit",
    ];

    loop {
        let choice = Select::new("Select a category:", categories.clone())
            .prompt()
            .unwrap_or_else(|_| "0. Exit");

        match choice {
            "1. Getting Started" => getting_started::run(),
            "2. Help" => help::run(),
            "3. Personalization" => personalization::run(),
            "4. Credentials" => credentials::run(),
            "5. Projects" => projects::run(),
            "6. IAM" => iam::run(),
            "7. Docker & GKE" => docker_gke::run(),
            "8. VM & Compute Engine" => vm_compute::run(),
            "9. Serverless & App Engine" => serverless::run(),
            "10. Miscellaneous" => miscellaneous::run(),
            "0. Exit" => {
                println!("Exiting...");
                exit(0);
            }
            _ => println!("Invalid selection, try again."),
        }
    }
}
