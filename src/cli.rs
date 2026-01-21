use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        name: Option<String>,

        local_port: Option<u16>,
        remote_host: Option<String>,
        remote_port: Option<u16>,

        ssh_host: Option<String>,
        ssh_port: Option<u16>,
        ssh_user: Option<String>,
        ssh_key_path: Option<String>,

        retry_on_failure: Option<bool>,
    },
    Up {
        name: String,
    },
    Ls {},
}
