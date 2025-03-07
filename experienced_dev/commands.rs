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

pub fn handle_command(
    command: &str,
    args: &HashMap<String, String>,
    log_file: &mut BufWriter<std::fs::File>
) {
    let cmd_parts: Vec<&str> = command.split_whitespace().collect();

    if cmd_parts.is_empty() {
        return;
    }

    match cmd_parts[0] {
        "help" => display_help(),
        "config" => handle_config(cmd_parts.get(1), args, log_file),
        "list" => {
            if cmd_parts.len() > 1 {
                handle_list_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify what to list. Type 'help' for available commands.");
            }
        }
        "create" => {
            if cmd_parts.len() > 1 {
                handle_create_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify what to create. Type 'help' for available commands.");
            }
        }
        "delete" => {
            if cmd_parts.len() > 1 {
                handle_delete_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify what to delete. Type 'help' for available commands.");
            }
        }
        "start" => {
            if cmd_parts.len() > 1 {
                handle_start_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify what to start. Type 'help' for available commands.");
            }
        }
        "stop" => {
            if cmd_parts.len() > 1 {
                handle_stop_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify what to stop. Type 'help' for available commands.");
            }
        }
        "deploy" => {
            if cmd_parts.len() > 1 {
                handle_deploy_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify what to deploy. Type 'help' for available commands.");
            }
        }
        "ssh" => compute::ssh_to_instance(args, log_file),
        "bq" => {
            if cmd_parts.len() > 1 {
                handle_bq_command(cmd_parts[1], cmd_parts.get(2), args, log_file);
            } else {
                println!(
                    "❌ Please specify a BigQuery operation. Type 'help' for available commands."
                );
            }
        }
        "iam" => {
            if cmd_parts.len() > 1 {
                handle_iam_command(cmd_parts[1], args, log_file);
            } else {
                println!("❌ Please specify an IAM operation. Type 'help' for available commands.");
            }
        }
        _ => println!("❌ Unknown command! Type 'help' for available commands."),
    }
}

fn display_help() {
    println!("Available commands:");
    println!("  login - Authenticate with Google Cloud");
    println!("  config - Configure gcloud defaults");
    println!("  list - List various resources");
    println!("    list projects - List all available projects");
    println!(
        "    list instances - List all VM instances [Arguments: --ZONE (default: us-central1-a)]"
    );
    println!("    list buckets - List all Cloud Storage buckets");
    println!(
        "    list clusters - List all GKE clusters [Arguments: --ZONE (default: us-central1-a)]"
    );
    println!(
        "    list services - List all Cloud Run services [Arguments: --REGION (default: us-central1)]"
    );
    println!("    list topics - List all Pub/Sub topics");
    println!("    list datasets - List all BigQuery datasets");
    println!("  create - Create resources");
    println!(
        "    create project - Create a new project [Arguments: --PROJECT_ID (default: nexus-cli-12), --PROJECT_NAME (default: nexus-2)]"
    );
    println!(
        "    create instance - Create a VM instance [Arguments: --INSTANCE_NAME, --ZONE, --MACHINE_TYPE]"
    );
    println!("    create bucket - Create a Cloud Storage bucket [Arguments: --BUCKET_NAME]");
    println!("    create cluster - Create a GKE cluster [Arguments: --CLUSTER_NAME, --ZONE]");
    println!("    create topic - Create a Pub/Sub topic [Arguments: --TOPIC_NAME]");
    println!("    create sql - Create a Cloud SQL instance [Arguments: --SQL_INSTANCE, --REGION]");
    println!("  delete - Delete resources");
    println!("    delete project - Delete a project [Arguments: --PROJECT_ID]");
    println!("    delete instance - Delete a VM instance [Arguments: --INSTANCE_NAME, --ZONE]");
    println!("    delete bucket - Delete a Cloud Storage bucket [Arguments: --BUCKET_NAME]");
    println!("  start - Start resources");
    println!("    start instance - Start a VM instance [Arguments: --INSTANCE_NAME, --ZONE]");
    println!("    start sql - Start a Cloud SQL instance [Arguments: --SQL_INSTANCE]");
    println!("  stop - Stop resources");
    println!("    stop instance - Stop a VM instance [Arguments: --INSTANCE_NAME, --ZONE]");
    println!("    stop sql - Stop a Cloud SQL instance [Arguments: --SQL_INSTANCE]");
    println!("  deploy - Deploy resources");
    println!(
        "    deploy function - Deploy a Cloud Function [Arguments: --FUNCTION_NAME, --RUNTIME]"
    );
    println!(
        "    deploy service - Deploy a Cloud Run service [Arguments: --SERVICE_NAME, --IMAGE_NAME, --REGION]"
    );
    println!("  ssh - SSH into a VM instance [Arguments: --INSTANCE_NAME, --ZONE]");
    println!("  bq - BigQuery operations");
    println!("    bq query - Run a BigQuery SQL query [Arguments: --QUERY_STRING]");
    println!("    bq mk table - Create a BigQuery table [Arguments: --DATASET_NAME, --TABLE_NAME]");
    println!("  iam - IAM operations");
    println!("    iam list - List IAM policies [Arguments: --PROJECT_ID]");
    println!("    iam add - Add IAM role binding [Arguments: --MEMBER, --ROLE, --PROJECT_ID]");
    println!("  exit - Exit the terminal");
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
