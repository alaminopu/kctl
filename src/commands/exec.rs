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
                // Try sh first (works on alpine/distroless), fall back to bash
                let status = Command::new("kubectl")
                    .arg("exec")
                    .arg("-it")
                    .arg(&pods[0])
                    .arg("-n")
                    .arg(namespace)
                    .arg("--")
                    .arg("sh")
                    .status()
                    .expect("Failed exec to pod");

                if !status.success() {
                    // Fallback to bash if sh is not available
                    Command::new("kubectl")
                        .arg("exec")
                        .arg("-it")
                        .arg(&pods[0])
                        .arg("-n")
                        .arg(namespace)
                        .arg("--")
                        .arg("bash")
                        .status()
                        .expect("Failed exec to pod");
                }
            } else {
                println!("Invalid app name or namespace?");
                std::process::exit(1);
            }
        }
    }
}
