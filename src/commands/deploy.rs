use std::process::{Command, Stdio};
use std::io::Write;

pub fn execute(app: Option<&str>, namespace: &str) {
    match app {
        None => {
            let status = Command::new("kubectl")
            .arg("get")
            .arg("deployment")
            .arg("-n")
            .arg(namespace)
            .status()
            .expect("failed to execute process");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        },
        Some(app_name) => {
            let cmd = Command::new("kubectl")
            .arg("get")
            .arg("deployment")
            .arg("-n")
            .arg(namespace)
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to get pod list");

            let mut child = Command::new("grep")
                .arg(app_name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed search with app name");

            {
                let stdin = child.stdin.as_mut().expect("Failed to open stdin");
                stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
            }
            let output = child.wait_with_output().expect("Failed to read stdout");
            if output.stdout.is_empty() {
                println!("No deployments found matching '{}' in namespace '{}'", app_name, namespace);
                std::process::exit(1);
            }
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
