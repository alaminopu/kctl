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

        let mut namespace = matches.value_of("namespace").unwrap();

        // check if -n was passed by user
        let is_namespace_passed: bool = if matches.occurrences_of("namespace") == 0 { false } else { true };
        if !namespace_from_config.is_empty() && !is_namespace_passed{
            namespace = &namespace_from_config;
        }

        Args {
            command: matches.value_of("command").unwrap(),
            app: matches.value_of("app"),
            port: matches.value_of("port"),
            namespace: namespace.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, App};
    
    #[test]
    fn test_parse() {
        let  app = App::new("kctl")
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
            .default_value("default"));

         let matches = app.get_matches_from(vec!["kctl", "pod", "food", "-n", "kube-system"]);

         let arguments = Args::parse(&matches);
         assert_eq!(arguments.command, "pod");
         assert_eq!(arguments.port, None);
         assert_eq!(arguments.app, Some("food"));
         assert_eq!(arguments.namespace, "kube-system");

    }
}
