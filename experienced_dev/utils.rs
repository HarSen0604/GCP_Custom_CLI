use std::collections::HashMap;
use std::io::{ BufWriter, Write };
use std::process::{ Command, Stdio };

pub fn run_command(cmd: &str, args: &[&str], log_file: &mut BufWriter<std::fs::File>) -> String {
    writeln!(log_file, "Running: {} {}", cmd, args.join(" ")).unwrap();
    println!("Executing: {} {}", cmd, args.join(" "));

    let output = Command::new("sh")
        .args(&["-c", &format!("{} {}", cmd, args.join(" "))])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stdout.is_empty() {
                writeln!(log_file, "Output:\n{}", stdout).unwrap();
                println!("{}", stdout);
                return stdout.to_string();
            }
            if !stderr.is_empty() {
                writeln!(log_file, "Error:\n{}", stderr).unwrap();
                println!("\x1b[31mError:\x1b[0m {}", stderr);
            }
        }
        Err(e) => {
            writeln!(log_file, "Failed to execute: {}\nError: {}", cmd, e).unwrap();
            println!("\x1b[31mFailed to execute:\x1b[0m {}\n\x1b[31mError:\x1b[0m {}", cmd, e);
        }
    }
    String::new()
}

pub fn check_login(log_file: &mut BufWriter<std::fs::File>) -> bool {
    let output = run_command("gcloud", &["auth", "list", "--format=json"], log_file);
    output.contains("\"account\":")
}

pub fn parse_args(input: &str, defaults: &HashMap<&str, &str>) -> HashMap<String, String> {
    let mut args_map = defaults
        .iter()
        .map(|(&k, &v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i].starts_with('-') && i + 1 < tokens.len() {
            args_map.insert(
                tokens[i].trim_start_matches('-').to_string(),
                tokens[i + 1].to_string()
            );
            i += 1;
        }
        i += 1;
    }
    args_map
}
