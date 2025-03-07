use std::process::Command;
use colored::*;

pub fn execute_command(cmd: &str, args: &[&str]) {
    println!("{}", "Executing...".yellow());
    
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("{}", format!("✅ Successfully executed: {} {}", cmd, args.join(" ")).green());
    } else {
        eprintln!("{}", format!("❌ Error executing: {} {}", cmd, args.join(" ")).red());
    }
}
