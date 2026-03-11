extern crate assert_cmd;


#[cfg(test)]
mod integration {
    use std::process::Command;
    use assert_cmd::prelude::*;

    // ── Root / global flags ──────────────────────────────────────

    #[test]
    fn calling_root_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .assert()
            .failure();
    }

    #[test]
    fn calling_with_invalid_command() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["invalid"])
            .assert()
            .failure();
    }

    #[test]
    fn calling_help_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["--help"])
            .assert()
            .success()
            .stdout(predicates::str::contains("Kubernetes CLI wrapper"));
    }

    #[test]
    fn calling_version_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["--version"])
            .assert()
            .success()
            .stdout(predicates::str::contains("0.2.1"));
    }

    // ── log command ──────────────────────────────────────────────

    #[test]
    fn calling_log_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["log"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    // ── exec command ─────────────────────────────────────────────

    #[test]
    fn calling_exec_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["exec"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    // ── forward command ──────────────────────────────────────────

    #[test]
    fn calling_forward_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["forward"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    // ── set-namespace command ────────────────────────────────────

    #[test]
    fn calling_set_namespace_sets_value() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["set-namespace", "-n", "integration-test-ns"])
            .assert()
            .success()
            .stdout(predicates::str::contains("Sucessfully set namespace"));
    }

    #[test]
    fn calling_set_namespace_with_default() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["set-namespace"])
            .assert()
            .success()
            .stdout(predicates::str::contains("Sucessfully set namespace default"));
    }

    // ── namespace flag ───────────────────────────────────────────

    #[test]
    fn log_respects_namespace_flag() {
        // Even with -n specified, log still requires an app name
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["log", "-n", "custom-ns"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn exec_respects_namespace_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["exec", "-n", "custom-ns"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn forward_respects_namespace_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["forward", "-n", "custom-ns"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }
}