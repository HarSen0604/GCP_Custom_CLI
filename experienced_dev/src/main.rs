mod utils;
mod commands;
mod config;
mod fzf;

use std::fs::OpenOptions;
use std::io::{ BufWriter, stdin, Write };
use crate::utils::{ run_command, check_login, parse_args };
use crate::commands::handle_command;
use crate::config::DEFAULT_VALUES;
use std::collections::HashMap;
use crate::fzf::search_log_with_fzf;

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

        if command == "fzf" {
            match search_log_with_fzf(log_file_path) {
                Ok(Some(selected_command)) => {
                    println!("Selected command: {}", selected_command);
                    let default_values: HashMap<String, String> = DEFAULT_VALUES.iter()
                        .map(|(&k, &v)| (k.to_string(), v.to_string()))
                        .collect();

                    handle_command(&selected_command, &default_values, &mut log_file);

                    continue;
                }
                Ok(None) => println!("No command selected."),
                Err(e) => eprintln!("Error: {}", e),
            }
            continue;
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

        // Log the executed command
        writeln!(log_file, "{}", command).expect("Failed to write to log file");

        println!("✅ Task completed.");
    }
}
