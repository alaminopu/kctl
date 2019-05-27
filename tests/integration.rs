extern crate assert_cmd;


#[cfg(test)]
mod integration {
    use std::process::Command;
    use assert_cmd::prelude::*;

    
    #[test]
    fn calling_root_without_args() {

        Command::cargo_bin("kctl")
            .unwrap()
            .assert()
            .failure();
    }

    #[test]
    fn calling_log_without_args() {

        Command::cargo_bin("kctl")
            .unwrap()
            .args(&["log"])
            .assert()
            .success()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn calling_exec_without_args() {

        Command::cargo_bin("kctl")
            .unwrap()
            .args(&["exec"])
            .assert()
            .success()
            .stdout("Missing app name!\n");
    }

    #[test]
    fn calling_forward_without_args() {

        Command::cargo_bin("kctl")
            .unwrap()
            .args(&["forward"])
            .assert()
            .success()
            .stdout("Missing app name!\n");
    }
}