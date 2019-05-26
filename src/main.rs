extern crate clap;

use clap::{Arg, App};
use std::process::{Command, Stdio};
use std::io::Write;

mod config;
mod args;

use crate::args::Args;
use crate::config::set;

fn main() {
    let matches = App::new("kctl")
        .version("0.2.1")
        .author("Md. Al-Amin <alaminopu.me@gmail.com>")
        .about("Kubernetes CLI wrapper for making things easier!")
        .arg(Arg::with_name("command")
            .help("Input command you want to run!")
            .index(1)
            .possible_values(&["pod", "svc", "deploy", "log", "exec", "forward", "set-namespace"])
            .required(true))
        .arg(Arg::with_name("app")
            .help("Get specific app pods")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .help("Get port number for port forwarding")
            .takes_value(true))
        .arg(Arg::with_name("namespace")
            .help("Specify the namespace to work on")
            .takes_value(true)
            .required(false)
            .short("n")
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
                    Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(&namespace)
                    .status()
                    .expect("failed to execute process");
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
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
        },
        "svc" => {
            match app {
                None => {
                    Command::new("kubectl")
                    .arg("get")
                    .arg("svc")
                    .arg("-n")
                    .arg(&namespace)
                    .status()
                    .expect("failed to execute process");
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
                    print!("{}", String::from_utf8_lossy(&output.stdout));

                }
            }
        },
        "deploy" => {
            match app {
                None => {
                    Command::new("kubectl")
                    .arg("get")
                    .arg("deployment")
                    .arg("-n")
                    .arg(&namespace)
                    .status()
                    .expect("failed to execute process");
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
                    print!("{}", String::from_utf8_lossy(&output.stdout));

                }
            }
        },
        "log" => {
            match app {
                None => {
                    println!("Missing app name!");
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
                    }
                    }
            }
        },
        "exec" => {
            match app {
                None => {
                    println!("Missing app name!");
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
                    }
                    
                }
            }
        },
        "forward" => {
            match app {
                None => {
                    println!("Missing app name!");
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
                    }
                }
            }
        },
        _      => unreachable!()
    }

}
