use crate::config;

pub fn execute(namespace: &str) {
    config::set("KCTL_NAMESPACE", namespace);
}
