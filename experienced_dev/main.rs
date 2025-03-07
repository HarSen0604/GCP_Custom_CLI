mod utils;
mod commands;
mod config;

use std::fs::OpenOptions;
use std::io::{ BufWriter, stdin };
use crate::utils::run_command;
use crate::utils::check_login;
use crate::utils::parse_args;
use crate::commands::handle_command;
use crate::config::DEFAULT_VALUES;

fn main() {
    let log_file_path = "task_log.txt";
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Failed to open log file");
    let mut log_file = BufWriter::new(file);

    println!("\n🌍 Welcome to Google Cloud Task Terminal!");
    let mut is_logged_in = check_login(&mut log_file);
    if !is_logged_in {
        println!("Type 'login' to authenticate with Google Cloud.");
    } else {
        println!("✅ Already logged in!");
    }

    loop {
        print!("gcloud-terminal> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        let command = input.trim();

        if command == "exit" {
            println!("🔚 Exiting Google Cloud Task Terminal...");
            break;
        }
        if command == "login" {
            if is_logged_in {
                println!("✅ Already logged in!");
            } else {
                println!("🔑 Logging in...");
                run_command("gcloud", &["auth", "login"], &mut log_file);
                is_logged_in = check_login(&mut log_file);
                println!("{}", if is_logged_in {
                    "✅ Logged in!"
                } else {
                    "❌ Login failed. Try again."
                });
            }
            continue;
        }
        if !is_logged_in {
            println!("⚠️ Please login first using 'login'.");
            continue;
        }

        let args = parse_args(command, &DEFAULT_VALUES);
        handle_command(command, &args, &mut log_file);
        println!("✅ Task completed.");
    }
}
