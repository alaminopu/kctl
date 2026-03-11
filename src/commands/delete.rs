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
            let pods = kubectl::get_pod_names(
                app_name,
                namespace,
                &["--no-headers", "-o", "custom-columns=:metadata.name"],
            );

            if !pods.is_empty() {
                println!("Deleting pod '{}'...", &pods[0]);
                let status = Command::new("kubectl")
                    .arg("delete")
                    .arg("pod")
                    .arg(&pods[0])
                    .arg("-n")
                    .arg(namespace)
                    .status()
                    .expect("Failed to delete pod");
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
