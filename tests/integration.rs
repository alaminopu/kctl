extern crate assert_cmd;


#[cfg(test)]
mod integration {
    use std::process::Command;
    use assert_cmd::prelude::*;

    
    #[test]
    fn calling_root_without_args() {

        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .assert()
            .failure();
    }

    #[test]
    fn calling_log_without_args() {

        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["log"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn calling_exec_without_args() {

        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["exec"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn calling_forward_without_args() {

        Command::new(assert_cmd::cargo::cargo_bin!("kctl"))
            .args(&["forward"])
            .assert()
            .failure()
            .stdout("Missing app name!\n");
    }
}