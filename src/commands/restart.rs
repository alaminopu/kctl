extern crate exitcode;

use std::process::Command;

pub fn execute(app: Option<&str>, namespace: &str) {
    match app {
        None => {
            println!("Missing app name!");
            std::process::exit(exitcode::USAGE);
        }
        Some(app_name) => {
            let status = Command::new("kubectl")
                .arg("rollout")
                .arg("restart")
                .arg(format!("deployment/{}", app_name))
                .arg("-n")
                .arg(namespace)
                .status()
                .expect("Failed to restart deployment");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    }
}
