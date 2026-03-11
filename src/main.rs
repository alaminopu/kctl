use clap::{Arg, Command as ClapCommand};

mod config;
mod args;
mod commands;

use crate::args::Args;

fn main() {
    let matches = ClapCommand::new("kctl")
        .version("0.2.1")
        .author("Md. Al-Amin <alaminopu.me@gmail.com>")
        .about("Kubernetes CLI wrapper for making things easier!")
        .arg(Arg::new("command")
            .help("Input command you want to run!")
            .index(1)
            .value_parser(["pod", "svc", "deploy", "log", "exec", "forward", "set-namespace", "describe", "delete", "restart", "ctx"])
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
            // set-namespace uses the raw clap value, not the config-overridden one
            let raw_ns = matches.get_one::<String>("namespace").map(|s| s.as_str()).unwrap();
            commands::set_namespace::execute(raw_ns);
        },
        "pod"          => commands::pod::execute(app, &namespace),
        "svc"          => commands::svc::execute(app, &namespace),
        "deploy"       => commands::deploy::execute(app, &namespace),
        "log"          => commands::log::execute(app, &namespace),
        "exec"         => commands::exec::execute(app, &namespace),
        "forward"      => commands::forward::execute(app, port, &namespace),
        "describe"     => commands::describe::execute(app, &namespace),
        "delete"       => commands::delete::execute(app, &namespace),
        "restart"      => commands::restart::execute(app, &namespace),
        "ctx"          => commands::context::execute(app),
        _              => unreachable!()
    }
}
