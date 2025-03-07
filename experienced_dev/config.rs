use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_VALUES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("PROJECT_ID", "nexus-cli-12");
        m.insert("PROJECT_NAME", "nexus-2");
        m.insert("ZONE", "us-central1-a");
        m.insert("REGION", "us-central1");
        m.insert("INSTANCE_NAME", "default-instance");
        m.insert("MACHINE_TYPE", "e2-medium");
        m.insert("BUCKET_NAME", "default-bucket-123");
        m.insert("FUNCTION_NAME", "default-function");
        m.insert("RUNTIME", "nodejs18");
        m.insert("CLUSTER_NAME", "default-cluster");
        m.insert("SERVICE_NAME", "default-service");
        m.insert("TOPIC_NAME", "default-topic");
        m.insert("SQL_INSTANCE", "default-sql");
        m.insert("DATASET_NAME", "default_dataset");
        m.insert("TABLE_NAME", "default_table");
        m.insert("IMAGE_NAME", "default-image");
        m
    };
}
