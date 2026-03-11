extern crate exitcode;

use clap::{Arg, Command as ClapCommand};
use std::process::{Command, Stdio};
use std::io::Write;

mod config;
mod args;

use crate::args::Args;
use crate::config::set;

fn main() {
    let matches = ClapCommand::new("kctl")
        .version("0.2.1")
        .author("Md. Al-Amin <alaminopu.me@gmail.com>")
        .about("Kubernetes CLI wrapper for making things easier!")
        .arg(Arg::new("command")
            .help("Input command you want to run!")
            .index(1)
            .value_parser(["pod", "svc", "deploy", "log", "exec", "forward", "set-namespace"])
            .required(true))
        .arg(Arg::new("app")
            .help("Get specific app pods")
            .index(2)
            .action(clap::ArgAction::Set))
        .arg(Arg::new("port")
            .help("Get port number for port forwarding")
            .index(3)
            .action(clap::ArgAction::Set))
        .arg(Arg::new("namespace")
            .help("Specify the namespace to work on")
            .action(clap::ArgAction::Set)
            .required(false)
            .short('n')
            .long("namespace")
            .default_value("default"))
        .get_matches();
    
    let arguments = Args::parse(&matches);
    let Args {
        command,
        app,
        port,
        namespace
    } = arguments;
    
    match command {
        "set-namespace" => {
            set("KCTL_NAMESPACE", &namespace);
        },
        "pod" => {
            match app {
                None => {
                    let status = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&namespace)
                    .status()
                    .expect("failed to execute process");
                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                },
                Some(app_name) => {
                    let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&namespace)
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
                        stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
                    }
                    let output = child.wait_with_output().expect("Failed to read stdout");
                    if output.stdout.is_empty() {
                        println!("No pods found matching '{}' in namespace '{}'", app_name, namespace);
                        std::process::exit(1);
                    }
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
        },
        "svc" => {
            match app {
                None => {
                    let status = Command::new("kubectl")
                    .arg("get")
                    .arg("svc")
                    .arg("-n")
                    .arg(&namespace)
                    .status()
                    .expect("failed to execute process");
                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                },
                Some(app_name) => {
                    let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("svc")
                    .arg("-n")
                    .arg(&namespace)
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
                        stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
                    }
                    let output = child.wait_with_output().expect("Failed to read stdout");
                    if output.stdout.is_empty() {
                        println!("No services found matching '{}' in namespace '{}'", app_name, namespace);
                        std::process::exit(1);
                    }
                    print!("{}", String::from_utf8_lossy(&output.stdout));

                }
            }
        },
        "deploy" => {
            match app {
                None => {
                    let status = Command::new("kubectl")
                    .arg("get")
                    .arg("deployment")
                    .arg("-n")
                    .arg(&namespace)
                    .status()
                    .expect("failed to execute process");
                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                },
                Some(app_name) => {
                    let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("deployment")
                    .arg("-n")
                    .arg(&namespace)
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
                        stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
                    }
                    let output = child.wait_with_output().expect("Failed to read stdout");
                    if output.stdout.is_empty() {
                        println!("No deployments found matching '{}' in namespace '{}'", app_name, namespace);
                        std::process::exit(1);
                    }
                    print!("{}", String::from_utf8_lossy(&output.stdout));

                }
            }
        },
        "log" => {
            match app {
                None => {
                    println!("Missing app name!");
                    std::process::exit(exitcode::USAGE);
                },
                Some(app_name) => {
                    let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&namespace)
                    .arg("-o=name")
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
                        stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
                    }
                    let output = child.wait_with_output().expect("Failed to read stdout");

                    let x = String::from_utf8_lossy(&output.stdout);
                    let v: Vec<&str> = x.split_whitespace().collect();

                    if v.len() > 0 {
                        // Running logs 
                        Command::new("kubectl")
                            .arg("logs")
                            .arg("-f")
                            .arg(v[0])
                            .arg("-n")
                            .arg(namespace)
                            .status()
                            .expect("Failed show logs");
                    }else {
                        println!("Invalid app name or namespace?");
                        std::process::exit(1);
                    }
                    }
            }
        },
        "exec" => {
            match app {
                None => {
                    println!("Missing app name!");
                    std::process::exit(exitcode::USAGE);
                },
                Some(app_name) => {
                    let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&namespace)
                    .arg("--no-headers")
                    .arg("-o")
                    .arg("custom-columns=:metadata.name")
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
                        stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
                    }
                    let output = child.wait_with_output().expect("Failed to read stdout");

                    let x = String::from_utf8_lossy(&output.stdout);
                    let v: Vec<&str> = x.split_whitespace().collect();
                    
                    if v.len() > 0 {
                        // exec
                        Command::new("kubectl")
                            .arg("exec")
                            .arg("-it")
                            .arg(v[0])
                            .arg("-n")
                            .arg(namespace)
                            .arg("bash")
                            .status()
                            .expect("Failed exec to pod");
                    }else {
                        println!("Invalid app name or namespace?");
                        std::process::exit(1);
                    }
                    
                }
            }
        },
        "forward" => {
            match app {
                None => {
                    println!("Missing app name!");
                    std::process::exit(exitcode::USAGE);
                },
                Some(app_name) => {
                    let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&namespace)
                    .arg("--no-headers")
                    .arg("-o")
                    .arg("custom-columns=:metadata.name")
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
                        stdin.write_all(&cmd.stdout).expect("Failed to write to stdin");
                    }
                    let output = child.wait_with_output().expect("Failed to read stdout");

                    let x = String::from_utf8_lossy(&output.stdout);
                    let v: Vec<&str> = x.split_whitespace().collect();
                    
                    if v.len() > 0 {
                        match port {
                            None => {
                                println!("Missing port number");
                                std::process::exit(exitcode::USAGE);
                            },
                            Some(port_number) => {
                                // exec
                                Command::new("kubectl")
                                    .arg("port-forward")
                                    .arg(v[0])
                                    .arg(port_number)
                                    .arg("-n")
                                    .arg(namespace)
                                    .status()
                                    .expect("Failed port-forward");
                            }
                        }
                    }else {
                        println!("Invalid app name or namespace?");
                        std::process::exit(1);
                    }
                }
            }
        },
        _      => unreachable!()
    }

}
