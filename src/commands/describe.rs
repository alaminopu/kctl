extern crate exitcode;

use std::process::Command;
use super::kubectl;

pub fn execute(app: Option<&str>, namespace: &str) {
    match app {
        None => {
            println!("Missing app name!");
            std::process::exit(exitcode::USAGE);
        }
        Some(app_name) => {
            let pods = kubectl::get_pod_names(app_name, namespace, &["-o=name"]);

            if !pods.is_empty() {
                let status = Command::new("kubectl")
                    .arg("describe")
                    .arg(&pods[0])
                    .arg("-n")
                    .arg(namespace)
                    .status()
                    .expect("Failed to describe pod");
                if !status.success() {
                    std::process::exit(status.code().unwrap_or(1));
                }
            } else {
                println!("No pods found matching '{}' in namespace '{}'", app_name, namespace);
                std::process::exit(1);
            }
        }
    }
}
