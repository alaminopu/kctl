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
