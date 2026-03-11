extern crate exitcode;

use std::process::{Command, Stdio};
use std::io::Write;

pub fn execute(app: Option<&str>, namespace: &str) {
    match app {
        None => {
            println!("Missing app name!");
            std::process::exit(exitcode::USAGE);
        },
        Some(app_name) => {
            let cmd = Command::new("kubectl")
            .arg("get")
            .arg("pods")
            .arg("-n")
            .arg(namespace)
            .arg("--no-headers")
            .arg("-o")
            .arg("custom-columns=:metadata.name")
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

            let x = String::from_utf8_lossy(&output.stdout);
            let v: Vec<&str> = x.split_whitespace().collect();

            if v.len() > 0 {
                // exec
                Command::new("kubectl")
                    .arg("exec")
                    .arg("-it")
                    .arg(v[0])
                    .arg("-n")
                    .arg(namespace)
                    .arg("bash")
                    .status()
                    .expect("Failed exec to pod");
            } else {
                println!("Invalid app name or namespace?");
                std::process::exit(1);
            }
        }
    }
}
