use super::kubectl;

pub fn execute(app: Option<&str>, namespace: &str) {
    if !kubectl::get_filtered("ingress", app, namespace) {
        let app_name = app.unwrap_or("unknown");
        println!("No ingresses found matching '{}' in namespace '{}'", app_name, namespace);
        std::process::exit(1);
    }
}
