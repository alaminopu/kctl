extern crate clap;

use clap::{Arg, App};
use std::process::{Command, Stdio};
use std::io::Write;

mod config;

fn main() {

    let matches = App::new("kctl")
        .version("0.1.0")
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
            .default_value("default")
            .long("namespace"))
        .get_matches();
    
    let namespace_from_config = config::get();
    let mut namespace = matches.value_of("namespace").unwrap();
    if !namespace_from_config.is_empty() && namespace == "default" {
        namespace = &namespace_from_config
    }
    
    // NOTE: it's safe to call unwrap() because the arg is required
    match matches.value_of("command").unwrap() {
        "set-namespace" => {
            if namespace != "default" {
                use self::config::set;
                set("KCTL_NAMESPACE", namespace);
            }else {
                println!("use -n <namespace> to set a namespace!");
            }
        },
        "pod" => {
            if matches.is_present("app"){
                let app = matches.value_of("app").unwrap();
                
                let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(namespace)
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to get pod list");

                let mut child = Command::new("grep")
                    .arg(app)
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
                
            }else {
                Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(namespace)
                    .status()
                    .expect("failed to execute process");
            }
           
        },
        "svc" => {
             if matches.is_present("app"){
                let app = matches.value_of("app").unwrap();
                
                let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("svc")
                    .arg("-n")
                    .arg(namespace)
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to get pod list");

                let mut child = Command::new("grep")
                    .arg(app)
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
                
            }else {

                Command::new("kubectl")
                    .arg("get")
                    .arg("svc")
                    .arg("-n")
                    .arg(namespace)
                    .status()
                    .expect("failed to execute process");
            }
        },
        "deploy" => {
            if matches.is_present("app"){
                let app = matches.value_of("app").unwrap();
                
                let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("deployment")
                    .arg("-n")
                    .arg(namespace)
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to get pod list");

                let mut child = Command::new("grep")
                    .arg(app)
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
                
            }else {
                Command::new("kubectl")
                    .arg("get")
                    .arg("deployment")
                    .arg("-n")
                    .arg(namespace)
                    .status()
                    .expect("failed to execute process");
            }
        },
        "log" => {
            if matches.is_present("app"){
                let app = matches.value_of("app").unwrap();
                
                let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(namespace)
                    .arg("-o=name")
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to get pod list");

                let mut child = Command::new("grep")
                    .arg(app)
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
                
            }else {
                println!("Missing app name!");
            }
        },
        "exec" => {
            if matches.is_present("app"){
                let app = matches.value_of("app").unwrap();
                
                let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(namespace)
                    .arg("--no-headers")
                    .arg("-o")
                    .arg("custom-columns=:metadata.name")
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to get pod list");

                let mut child = Command::new("grep")
                    .arg(app)
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
                
                
            }else {
                println!("Missing app name!");
            }
        },
        "forward" => {
            if matches.is_present("app"){
                let app = matches.value_of("app").unwrap();
                
                let cmd = Command::new("kubectl")
                    .arg("get")
                    .arg("pods")
                    .arg("-n")
                    .arg(namespace)
                    .arg("--no-headers")
                    .arg("-o")
                    .arg("custom-columns=:metadata.name")
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to get pod list");

                let mut child = Command::new("grep")
                    .arg(app)
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
                    if matches.is_present("port"){
                        let port = matches.value_of("port").unwrap();

                        // exec
                        Command::new("kubectl")
                            .arg("port-forward")
                            .arg(v[0])
                            .arg(port)
                            .arg("-n")
                            .arg(namespace)
                            .status()
                            .expect("Failed port-forward");
                    }else {
                        println!("Missing port number");
                    }
                }else {
                    println!("Invalid app name or namespace?");
                }
                
                
            }else {
                println!("Missing app name!");
            }
        },
        _      => unreachable!()
    }

}
