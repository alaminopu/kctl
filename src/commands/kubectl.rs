use std::process::{Command, Stdio};
use std::io::Write;

/// Run `kubectl get <resource> -n <namespace>` and optionally filter output by app name via grep.
/// If `app` is `None`, the raw kubectl output is printed directly.
/// If `app` is `Some(name)`, output is piped through grep and matching lines are printed.
///
/// Returns `true` if the command succeeded and produced output (or no app filter was needed).
/// Returns `false` if no matches were found (caller should handle the error message).
pub fn get_filtered(resource: &str, app: Option<&str>, namespace: &str) -> bool {
    match app {
        None => {
            let status = Command::new("kubectl")
                .arg("get")
                .arg(resource)
                .arg("-n")
                .arg(namespace)
                .status()
                .expect("failed to execute process");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
            true
        }
        Some(app_name) => {
            let cmd = Command::new("kubectl")
                .arg("get")
                .arg(resource)
                .arg("-n")
                .arg(namespace)
                .stdout(Stdio::piped())
                .output()
                .expect("Failed to get resource list");

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
                return false;
            }
            print!("{}", String::from_utf8_lossy(&output.stdout));
            true
        }
    }
}

/// Run `kubectl get pods -n <namespace>` with custom extra args and grep for the app name.
/// Returns a list of matching pod names (one per line from grep output).
pub fn get_pod_names(app_name: &str, namespace: &str, extra_args: &[&str]) -> Vec<String> {
    let mut cmd = Command::new("kubectl");
    cmd.arg("get")
        .arg("pods")
        .arg("-n")
        .arg(namespace);
    for arg in extra_args {
        cmd.arg(arg);
    }
    let output = cmd
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
        stdin.write_all(&output.stdout).expect("Failed to write to stdin");
    }
    let grep_output = child.wait_with_output().expect("Failed to read stdout");

    let text = String::from_utf8_lossy(&grep_output.stdout);
    text.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
