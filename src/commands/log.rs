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
                // Running logs
                Command::new("kubectl")
                    .arg("logs")
                    .arg("-f")
                    .arg(&pods[0])
                    .arg("-n")
                    .arg(namespace)
                    .status()
                    .expect("Failed show logs");
            } else {
                println!("Invalid app name or namespace?");
                std::process::exit(1);
            }
        }
    }
}
