use std::process::Command;

pub fn execute(app: Option<&str>) {
    match app {
        None => {
            // List all contexts
            let status = Command::new("kubectl")
                .arg("config")
                .arg("get-contexts")
                .status()
                .expect("Failed to get contexts");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Some(context_name) => {
            // Switch to specified context
            let status = Command::new("kubectl")
                .arg("config")
                .arg("use-context")
                .arg(context_name)
                .status()
                .expect("Failed to switch context");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    }
}
