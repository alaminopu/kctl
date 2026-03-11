extern crate exitcode;

use std::process::Command;
use super::kubectl;

pub fn execute(app: Option<&str>, port: Option<&str>, namespace: &str) {
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
                match port {
                    None => {
                        println!("Missing port number");
                        std::process::exit(exitcode::USAGE);
                    }
                    Some(port_number) => {
                        // port-forward
                        Command::new("kubectl")
                            .arg("port-forward")
                            .arg(&pods[0])
                            .arg(port_number)
                            .arg("-n")
                            .arg(namespace)
                            .status()
                            .expect("Failed port-forward");
                    }
                }
            } else {
                println!("Invalid app name or namespace?");
                std::process::exit(1);
            }
        }
    }
}
