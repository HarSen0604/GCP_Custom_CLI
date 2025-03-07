mod compute;
mod storage;
mod functions;
mod containers;
mod run;
mod pubsub;
mod bigquery;
mod iam;
mod sql;

use std::collections::HashMap;
use std::io::BufWriter;
use crate::utils::run_command;

// Function to calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let m = s1_chars.len();
    let n = s2_chars.len();

    // Create a matrix of size (m+1) x (n+1)
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize first row and column
    for i in 0..=m {
        dp[i][0] = i;
    }

    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill the matrix
    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };

            dp[i][j] = std::cmp::min(
                std::cmp::min(
                    dp[i - 1][j] + 1, // deletion
                    dp[i][j - 1] + 1 // insertion
                ),
                dp[i - 1][j - 1] + cost // substitution
            );
        }
    }

    dp[m][n]
}

// Function to get a list of all available commands
fn get_all_commands() -> Vec<String> {
    vec![
        "help".to_string(),
        "login".to_string(),
        "config".to_string(),
        "list".to_string(),
        "list projects".to_string(),
        "list instances".to_string(),
        "list buckets".to_string(),
        "list clusters".to_string(),
        "list services".to_string(),
        "list topics".to_string(),
        "list datasets".to_string(),
        "create".to_string(),
        "create project".to_string(),
        "create instance".to_string(),
        "create bucket".to_string(),
        "create cluster".to_string(),
        "create topic".to_string(),
        "create sql".to_string(),
        "delete".to_string(),
        "delete project".to_string(),
        "delete instance".to_string(),
        "delete bucket".to_string(),
        "start".to_string(),
        "start instance".to_string(),
        "start sql".to_string(),
        "stop".to_string(),
        "stop instance".to_string(),
        "stop sql".to_string(),
        "deploy".to_string(),
        "deploy function".to_string(),
        "deploy service".to_string(),
        "ssh".to_string(),
        "bq".to_string(),
        "bq query".to_string(),
        "bq mk table".to_string(),
        "iam".to_string(),
        "iam list".to_string(),
        "iam add".to_string(),
        "exit".to_string()
    ]
}

// Function to find the closest command match
fn find_similar_command(input: &str) -> Option<String> {
    let all_commands = get_all_commands();
    let mut best_match = None;
    let mut min_distance = usize::MAX;

    println!("\x1b[35m[DEBUG] Input command: '{}'\x1b[0m", input);

    for cmd in &all_commands {
        let distance = levenshtein_distance(input, cmd);
        let threshold = std::cmp::max(2, cmd.len() / 4);

        println!("\x1b[35m[DEBUG] Checking '{}', distance: {}\x1b[0m", cmd, distance);

        if distance < min_distance && distance <= threshold {
            min_distance = distance;
            best_match = Some(cmd.clone());
        }
    }

    // If no good match found, try prefix matching
    if best_match.is_none() {
        for cmd in &all_commands {
            if cmd.starts_with(input) {
                println!("\x1b[35m[DEBUG] Found prefix match: '{}'\x1b[0m", cmd);
                return Some(cmd.clone());
            }
        }
    }

    println!("\x1b[35m[DEBUG] No suitable match found\x1b[0m");
    best_match
}

pub fn handle_command(
    command: &str,
    args: &HashMap<String, String>,
    log_file: &mut std::io::BufWriter<std::fs::File>
) {
    let cmd_parts: Vec<&str> = command.split_whitespace().collect();

    if cmd_parts.is_empty() {
        return;
    }

    // Check for help flags
    if cmd_parts.len() > 1 && cmd_parts[1] == "--help" {
        display_help(Some(cmd_parts[0]));
        return;
    }

    // Combine command parts for better matching
    let full_command = if cmd_parts.len() > 1 {
        format!("{} {}", cmd_parts[0], cmd_parts[1])
    } else {
        cmd_parts[0].to_string()
    };

    match cmd_parts[0] {
        "help" => {
            if cmd_parts.len() > 1 {
                display_help(Some(cmd_parts[1]));
            } else {
                display_help(None);
            }
        }
        "config" => handle_config(cmd_parts.get(1), args, log_file),
        "list" => {
            if cmd_parts.len() > 1 {
                handle_list_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify what to list. Use 'list --help' for available commands.\x1b[0m"
                );
            }
        }
        "create" => {
            if cmd_parts.len() > 1 {
                handle_create_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify what to create. Use 'create --help' for available commands.\x1b[0m"
                );
            }
        }
        "delete" => {
            if cmd_parts.len() > 1 {
                handle_delete_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify what to delete. Use 'delete --help' for available commands.\x1b[0m"
                );
            }
        }
        "start" => {
            if cmd_parts.len() > 1 {
                handle_start_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify what to start. Use 'start --help' for available commands.\x1b[0m"
                );
            }
        }
        "stop" => {
            if cmd_parts.len() > 1 {
                handle_stop_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify what to stop. Use 'stop --help' for available commands.\x1b[0m"
                );
            }
        }
        "deploy" => {
            if cmd_parts.len() > 1 {
                handle_deploy_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify what to deploy. Use 'deploy --help' for available commands.\x1b[0m"
                );
            }
        }
        "ssh" => compute::ssh_to_instance(args, log_file),
        "bq" => {
            if cmd_parts.len() > 1 {
                handle_bq_command(cmd_parts[1], cmd_parts.get(2), args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify a BigQuery operation. Use 'bq --help' for available commands.\x1b[0m"
                );
            }
        }
        "iam" => {
            if cmd_parts.len() > 1 {
                handle_iam_command(cmd_parts[1], args, log_file);
            } else {
                println!(
                    "\x1b[33m⚠️ Please specify an IAM operation. Use 'iam --help' for available commands.\x1b[0m"
                );
            }
        }
        _ => {
            // Check for similar commands
            if let Some(suggestion) = find_similar_command(&full_command) {
                println!("\x1b[31m❌ Unknown command: '{}'\x1b[0m", command);
                println!("\x1b[36m💡 Did you mean: '\x1b[32m{}\x1b[36m'?\x1b[0m", suggestion);
            } else {
                println!("\x1b[31m❌ Unknown command! Type 'help' for available commands.\x1b[0m");
            }
        }
    }
}
fn display_help(command: Option<&str>) {
    match command {
        None => {
            // Main help menu - only show top-level commands
            println!("\x1b[36mAvailable commands:\x1b[0m");
            println!("  \x1b[32mlogin\x1b[0m    - Authenticate with Google Cloud");
            println!("  \x1b[32mconfig\x1b[0m   - Configure gcloud defaults");
            println!("  \x1b[32mlist\x1b[0m     - List various resources");
            println!("  \x1b[32mcreate\x1b[0m   - Create resources");
            println!("  \x1b[32mdelete\x1b[0m   - Delete resources");
            println!("  \x1b[32mstart\x1b[0m    - Start resources");
            println!("  \x1b[32mstop\x1b[0m     - Stop resources");
            println!("  \x1b[32mdeploy\x1b[0m   - Deploy resources");
            println!("  \x1b[32mssh\x1b[0m      - SSH into a VM instance");
            println!("  \x1b[32mbq\x1b[0m       - BigQuery operations");
            println!("  \x1b[32miam\x1b[0m      - IAM operations");
            println!("  \x1b[32mexit\x1b[0m     - Exit the terminal");
            println!(
                "\nUse '\x1b[33m<command> --help\x1b[0m' for more information on specific commands"
            );
        }
        Some("list") => {
            println!("\x1b[36mList commands:\x1b[0m");
            println!("  \x1b[32mlist projects\x1b[0m        - List all available projects");
            println!("  \x1b[32mlist instances\x1b[0m       - List all VM instances");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --ZONE (default: us-central1-a)"
            );
            println!("  \x1b[32mlist buckets\x1b[0m         - List all Cloud Storage buckets");
            println!("  \x1b[32mlist clusters\x1b[0m        - List all GKE clusters");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --ZONE (default: us-central1-a)"
            );
            println!("  \x1b[32mlist services\x1b[0m        - List all Cloud Run services");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --REGION (default: us-central1)"
            );
            println!("  \x1b[32mlist topics\x1b[0m          - List all Pub/Sub topics");
            println!("  \x1b[32mlist datasets\x1b[0m        - List all BigQuery datasets");
        }
        Some("create") => {
            println!("\x1b[36mCreate commands:\x1b[0m");
            println!("  \x1b[32mcreate project\x1b[0m       - Create a new project");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --PROJECT_ID (default: nexus-cli-12)"
            );
            println!("                                    --PROJECT_NAME (default: nexus-2)");
            println!("  \x1b[32mcreate instance\x1b[0m      - Create a VM instance");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --INSTANCE_NAME, --ZONE, --MACHINE_TYPE"
            );
            println!("  \x1b[32mcreate bucket\x1b[0m        - Create a Cloud Storage bucket");
            println!("                         \x1b[33mArguments:\x1b[0m --BUCKET_NAME");
            println!("  \x1b[32mcreate cluster\x1b[0m       - Create a GKE cluster");
            println!("                         \x1b[33mArguments:\x1b[0m --CLUSTER_NAME, --ZONE");
            println!("  \x1b[32mcreate topic\x1b[0m         - Create a Pub/Sub topic");
            println!("                         \x1b[33mArguments:\x1b[0m --TOPIC_NAME");
            println!("  \x1b[32mcreate sql\x1b[0m           - Create a Cloud SQL instance");
            println!("                         \x1b[33mArguments:\x1b[0m --SQL_INSTANCE, --REGION");
        }
        Some("delete") => {
            println!("\x1b[36mDelete commands:\x1b[0m");
            println!("  \x1b[32mdelete project\x1b[0m       - Delete a project");
            println!("                         \x1b[33mArguments:\x1b[0m --PROJECT_ID");
            println!("  \x1b[32mdelete instance\x1b[0m      - Delete a VM instance");
            println!("                         \x1b[33mArguments:\x1b[0m --INSTANCE_NAME, --ZONE");
            println!("  \x1b[32mdelete bucket\x1b[0m        - Delete a Cloud Storage bucket");
            println!("                         \x1b[33mArguments:\x1b[0m --BUCKET_NAME");
        }
        Some("start") => {
            println!("\x1b[36mStart commands:\x1b[0m");
            println!("  \x1b[32mstart instance\x1b[0m       - Start a VM instance");
            println!("                         \x1b[33mArguments:\x1b[0m --INSTANCE_NAME, --ZONE");
            println!("  \x1b[32mstart sql\x1b[0m            - Start a Cloud SQL instance");
            println!("                         \x1b[33mArguments:\x1b[0m --SQL_INSTANCE");
        }
        Some("stop") => {
            println!("\x1b[36mStop commands:\x1b[0m");
            println!("  \x1b[32mstop instance\x1b[0m        - Stop a VM instance");
            println!("                         \x1b[33mArguments:\x1b[0m --INSTANCE_NAME, --ZONE");
            println!("  \x1b[32mstop sql\x1b[0m             - Stop a Cloud SQL instance");
            println!("                         \x1b[33mArguments:\x1b[0m --SQL_INSTANCE");
        }
        Some("deploy") => {
            println!("\x1b[36mDeploy commands:\x1b[0m");
            println!("  \x1b[32mdeploy function\x1b[0m      - Deploy a Cloud Function");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --FUNCTION_NAME, --RUNTIME"
            );
            println!("  \x1b[32mdeploy service\x1b[0m       - Deploy a Cloud Run service");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --SERVICE_NAME, --IMAGE_NAME, --REGION"
            );
        }
        Some("bq") => {
            println!("\x1b[36mBigQuery commands:\x1b[0m");
            println!("  \x1b[32mbq query\x1b[0m             - Run a BigQuery SQL query");
            println!("                         \x1b[33mArguments:\x1b[0m --QUERY_STRING");
            println!("  \x1b[32mbq mk table\x1b[0m          - Create a BigQuery table");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --DATASET_NAME, --TABLE_NAME"
            );
        }
        Some("iam") => {
            println!("\x1b[36mIAM commands:\x1b[0m");
            println!("  \x1b[32miam list\x1b[0m             - List IAM policies");
            println!("                         \x1b[33mArguments:\x1b[0m --PROJECT_ID");
            println!("  \x1b[32miam add\x1b[0m              - Add IAM role binding");
            println!(
                "                         \x1b[33mArguments:\x1b[0m --MEMBER, --ROLE, --PROJECT_ID"
            );
        }
        Some("ssh") => {
            println!("\x1b[36mSSH command:\x1b[0m");
            println!("  \x1b[32mssh\x1b[0m                  - SSH into a VM instance");
            println!("                         \x1b[33mArguments:\x1b[0m --INSTANCE_NAME, --ZONE");
        }
        Some(cmd) => {
            println!("\x1b[31mUnknown command:\x1b[0m {}", cmd);
            println!("Use 'help' to see available commands");
        }
    }
}
fn handle_config(
    subcommand: Option<&&str>,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    println!("Configuring gcloud defaults...");
    run_command("gcloud", &["config", "list"], log_file);

    if let Some(&"set") = subcommand {
        let project_id = &args["PROJECT_ID"];
        let zone = &args["ZONE"];
        let region = &args["REGION"];
        println!("Setting default configuration...");
        run_command("gcloud", &["config", "set", "project", project_id], log_file);
        run_command("gcloud", &["config", "set", "compute/zone", zone], log_file);
        run_command("gcloud", &["config", "set", "compute/region", region], log_file);
        println!("✅ Default configuration set!");
    }
}

fn handle_list_command(
    resource: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match resource {
        "projects" => {
            println!("Listing projects...");
            run_command("gcloud", &["projects", "list"], log_file);
        }
        "instances" => compute::list_instances(args, log_file),
        "buckets" => storage::list_buckets(log_file),
        "clusters" => containers::list_clusters(args, log_file),
        "services" => run::list_services(args, log_file),
        "topics" => pubsub::list_topics(log_file),
        "datasets" => bigquery::list_datasets(log_file),
        _ => println!("❌ Unknown list command. Type 'help' for available commands."),
    }
}

fn handle_create_command(
    resource: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match resource {
        "project" => {
            let project_id = &args["PROJECT_ID"];
            let project_name = &args["PROJECT_NAME"];
            println!("Creating project '{}' with name '{}'...", project_id, project_name);
            run_command(
                "gcloud",
                &["projects", "create", project_id, "--name", project_name],
                log_file
            );
        }
        "instance" => compute::create_instance(args, log_file),
        "bucket" => storage::create_bucket(args, log_file),
        "cluster" => containers::create_cluster(args, log_file),
        "topic" => pubsub::create_topic(args, log_file),
        "sql" => sql::create_sql_instance(args, log_file),
        _ => println!("❌ Unknown create command. Type 'help' for available commands."),
    }
}

fn handle_delete_command(
    resource: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match resource {
        "project" => {
            let project_id = &args["PROJECT_ID"];
            println!("Deleting project '{}'...", project_id);
            run_command("gcloud", &["projects", "delete", project_id], log_file);
        }
        "instance" => compute::delete_instance(args, log_file),
        "bucket" => storage::delete_bucket(args, log_file),
        _ => println!("❌ Unknown delete command. Type 'help' for available commands."),
    }
}

fn handle_start_command(
    resource: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match resource {
        "instance" => compute::start_instance(args, log_file),
        "sql" => sql::start_sql_instance(args, log_file),
        _ => println!("❌ Unknown start command. Type 'help' for available commands."),
    }
}

fn handle_stop_command(
    resource: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match resource {
        "instance" => compute::stop_instance(args, log_file),
        "sql" => sql::stop_sql_instance(args, log_file),
        _ => println!("❌ Unknown stop command. Type 'help' for available commands."),
    }
}

fn handle_deploy_command(
    resource: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match resource {
        "function" => functions::deploy_function(args, log_file),
        "service" => run::deploy_service(args, log_file),
        _ => println!("❌ Unknown deploy command. Type 'help' for available commands."),
    }
}

fn handle_bq_command(
    operation: &str,
    suboperation: Option<&&str>,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match operation {
        "query" => bigquery::run_query(args, log_file),
        "mk" => {
            if let Some(&"table") = suboperation {
                bigquery::create_table(args, log_file);
            } else {
                println!("❌ Unknown BigQuery make command. Type 'help' for available commands.");
            }
        }
        _ => println!("❌ Unknown BigQuery command. Type 'help' for available commands."),
    }
}

fn handle_iam_command(
    operation: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    match operation {
        "list" => iam::list_iam_policy(args, log_file),
        "add" => iam::add_iam_binding(args, log_file),
        _ => println!("❌ Unknown IAM command. Type 'help' for available commands."),
    }
}
