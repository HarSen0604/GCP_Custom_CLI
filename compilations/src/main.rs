// main.rs
mod commands;
use commands::*;
use std::collections::HashMap;
use std::io::{self, Write};
use colored::*;

fn main() {
    println!("🚀 Welcome to Rust GCloud Shell! Type 'help' for available commands.");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        
        let args: HashMap<String, String> = parts
            .filter_map(|arg| arg.split_once('='))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        match command {
            "help" => help::print_help(),
            "set_defaults" => set_defaults::execute(&args),
            "create_instance" => create_instance::execute(&args),
            "create_cluster" => create_cluster::execute(&args),
            "create_network" => create_network::execute(&args),
            "add_iam_policy" => add_iam_policy::execute(&args),
            "remove_iam_policy" => remove_iam_policy::execute(&args),
            "set_iam_policy" => set_iam_policy::execute(&args),
            "deploy_app" => deploy_app::execute(&args),
            "exit" => {
                println!("👋 Exiting...");
                break;
            }
            _ => { println!("{}", "❌ Unknown command. Type 'help' for available commands.".red()); }
        }
    }
}