use clap::ArgMatches;
use crate::config;

pub struct Args<'a> {
    pub command: &'a str,
    pub app: Option<&'a str>,
    pub port: Option<&'a str>,
    pub namespace: String
}
impl<'a> Args<'a> {
    pub fn parse(matches: &'a ArgMatches) -> Args<'a> {
        let namespace_from_config = config::get();

        let mut namespace = matches.get_one::<String>("namespace").map(|s| s.as_str()).unwrap();

        // check if -n was explicitly passed by user (not just the default)
        let is_namespace_explicit = matches
            .value_source("namespace")
            .map(|s| s == clap::parser::ValueSource::CommandLine)
            .unwrap_or(false);
        if !namespace_from_config.is_empty() && !is_namespace_explicit {
            namespace = &namespace_from_config;
        }

        Args {
            command: matches.get_one::<String>("command").map(|s| s.as_str()).unwrap(),
            app: matches.get_one::<String>("app").map(|s| s.as_str()),
            port: matches.get_one::<String>("port").map(|s| s.as_str()),
            namespace: namespace.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};
    
    #[test]
    fn test_parse() {
        let  app = Command::new("kctl")
        .version("1.0.0")
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
            .default_value("default"));

         let matches = app.get_matches_from(vec!["kctl", "pod", "food", "-n", "kube-system"]);

         let arguments = Args::parse(&matches);
         assert_eq!(arguments.command, "pod");
         assert_eq!(arguments.port, None);
         assert_eq!(arguments.app, Some("food"));
         assert_eq!(arguments.namespace, "kube-system");

    }
}
