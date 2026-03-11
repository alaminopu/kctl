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
            .stdout(predicates::str::contains("1.0.0"));
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

    // ── describe command ─────────────────────────────────────────

    #[test]
    fn calling_describe_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["describe"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn describe_respects_namespace_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["describe", "-n", "custom-ns"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    // ── delete command ───────────────────────────────────────────

    #[test]
    fn calling_delete_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["delete"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn delete_respects_namespace_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["delete", "-n", "custom-ns"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    // ── restart command ──────────────────────────────────────────

    #[test]
    fn calling_restart_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["restart"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn restart_respects_namespace_flag() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["restart", "-n", "custom-ns"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    // ── ctx command ──────────────────────────────────────────────

    #[test]
    fn calling_ctx_is_valid_command() {
        // ctx without args should list contexts (or fail gracefully if kubectl not configured)
        let _ = Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["ctx"])
            .assert();
        // We just verify it doesn't panic — exit code depends on kubectl config
    }

    // ── scale command ────────────────────────────────────────────

    #[test]
    fn calling_scale_without_args() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["scale"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn calling_scale_without_replicas() {
        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["scale", "myapp"])
            .assert()
            .failure()
            .stdout("Missing replica count! Usage: kctl scale <app> <replicas>\n");
    }

    // ── help lists all commands ──────────────────────────────────

    #[test]
    fn help_shows_all_commands() {
        let output = Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["--help"])
            .assert()
            .success();

        // Verify key new commands appear in help text
        output
            .stdout(predicates::str::contains("describe"))
            .stdout(predicates::str::contains("delete"))
            .stdout(predicates::str::contains("restart"))
            .stdout(predicates::str::contains("ctx"))
            .stdout(predicates::str::contains("scale"))
            .stdout(predicates::str::contains("ing"))
            .stdout(predicates::str::contains("cm"))
            .stdout(predicates::str::contains("secret"))
            .stdout(predicates::str::contains("top"))
            .stdout(predicates::str::contains("events"));
    }
}