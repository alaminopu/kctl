use super::kubectl;

pub fn execute(app: Option<&str>, namespace: &str) {
    match app {
        None => {
            let status = std::process::Command::new("kubectl")
                .arg("top")
                .arg("pods")
                .arg("-n")
                .arg(namespace)
                .status()
                .expect("Failed to get pod metrics");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Some(app_name) => {
            if !kubectl::get_filtered("pods", Some(app_name), namespace) {
                // Fall back: try kubectl top pod with the name directly
                let pods = kubectl::get_pod_names(
                    app_name,
                    namespace,
                    &["--no-headers", "-o", "custom-columns=:metadata.name"],
                );
                if !pods.is_empty() {
                    let status = std::process::Command::new("kubectl")
                        .arg("top")
                        .arg("pod")
                        .arg(&pods[0])
                        .arg("-n")
                        .arg(namespace)
                        .status()
                        .expect("Failed to get pod metrics");
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
}
