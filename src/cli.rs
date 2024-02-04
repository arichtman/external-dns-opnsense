

use clap::{arg, command, Parser};



// TODO: Update env use when issue is resolved https://github.com/clap-rs/clap/issues/3221
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, )]
pub struct Cli {
    #[arg(short, long, env = "EDNS_KEY", default_value = "")]
    pub key: String,
    #[arg(short = 's', long, env = "EDNS_SECRET", default_value = "")]
    pub secret: String,
    #[arg(short = 'u', long, env = "EDNS_FQDN", default_value = "")]
    pub fqdn: String,
    #[arg(short = 'd', long, action = clap::ArgAction::Append, env = "EDNS_DOMAIN", long_help = "May be specified multiple times.", default_values_t = vec!(String::from("local")))]
    pub domain: Vec<String>,
    /// Increments logging verbosity.
    #[arg(short, long, action = clap::ArgAction::Count, env = "EDNS_VERBOSE", long_help = "Optional. May be applied up to 4 times. Environment variable takes integer.")]
    pub verbose: u8,
}

pub fn get() -> Cli {
    Cli::parse()
}
