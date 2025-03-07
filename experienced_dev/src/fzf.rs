use std::fs::File;
use std::io::{ self, BufRead, Write };
use std::process::{ Command, Stdio };

pub fn search_log_with_fzf(path: &str) -> io::Result<Option<String>> {
    let log_file_path = path;
    let file = File::open(log_file_path)?;
    let reader = io::BufReader::new(file);

    let logs: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    if logs.is_empty() {
        return Ok(None);
    }

    let mut child = Command::new("fzf").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        for log in logs {
            writeln!(stdin, "{}", log)?;
        }
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(Some(selected))
    } else {
        Ok(None)
    }
}
