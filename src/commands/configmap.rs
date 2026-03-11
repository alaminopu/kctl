use super::kubectl;

pub fn execute(app: Option<&str>, namespace: &str) {
    if !kubectl::get_filtered("configmap", app, namespace) {
        let app_name = app.unwrap_or("unknown");
        println!("No configmaps found matching '{}' in namespace '{}'", app_name, namespace);
        std::process::exit(1);
    }
}
