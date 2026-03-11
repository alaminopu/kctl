extern crate exitcode;

use std::process::Command;

pub fn execute(app: Option<&str>, port: Option<&str>, namespace: &str) {
    match app {
        None => {
            println!("Missing app name!");
            std::process::exit(exitcode::USAGE);
        }
        Some(app_name) => {
            match port {
                None => {
                    println!("Missing replica count! Usage: kctl scale <app> <replicas>");
                    std::process::exit(exitcode::USAGE);
                }
                Some(replicas) => {
                    let status = Command::new("kubectl")
                        .arg("scale")
                        .arg(format!("deployment/{}", app_name))
                        .arg(format!("--replicas={}", replicas))
                        .arg("-n")
                        .arg(namespace)
                        .status()
                        .expect("Failed to scale deployment");
                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                }
            }
        }
    }
}
