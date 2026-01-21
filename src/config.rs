use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub tunnels: HashMap<String, Tunnel>,
}

#[derive(Clone, Deserialize, Serialize, Debug, Parser)]
pub struct Tunnel {
    // base
    pub name: String,

    // tunnel data
    #[arg(long)]
    pub local_port: u16,
    pub remote_host: String,
    #[arg(long)]
    pub remote_port: u16,

    // ssh data
    pub ssh_host: String,
    #[arg(long)]
    pub ssh_port: u16,
    pub ssh_user: String,
    pub ssh_key_path: Option<String>,

    pub retry_on_failure: Option<bool>,
}

impl Config {
    // Load a file from path
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // if let Some(proj_dirs) = ProjectDirs::from("", "", "rtun") {
        //     let config_dir = proj_dirs.config_dir();
        //     let config_path = config_dir.join("config.toml");

        //     // println!("path: {:?}", config_path);
        //     match fs::read_to_string(&config_path) {
        //         Ok(content) => match toml::from_str::<Config>(&content) {
        //             Ok(config) => Ok(config),
        //             Err(e) => Err(e)?,
        //         },
        //         Err(e) => {
        //             // let new_toml = toml::to_string(Config);
        //             Err(e)?
        //         }
        //     }
        // } else {
        //     Err("Cannot find project directory".into())
        // }
        if fs::metadata(path).is_err() {
            return Ok(Config {
                tunnels: HashMap::new(),
            });
        }

        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    // Get the tunnel config
    pub fn get(tunnel_name: &str) -> Result<Tunnel, Box<dyn std::error::Error>> {
        let read_config = Config::load("config.toml").expect("Failed to load config");
        let tunnel: Tunnel = read_config
            .tunnels
            .get(tunnel_name)
            .cloned()
            .ok_or(format!("Tunnel name: '{}' not found", tunnel_name))?;

        Ok(tunnel)
    }

    // Save a new tunnel to config.toml
    pub fn save(data: Tunnel) -> Result<(), Box<dyn std::error::Error>> {
        let mut read_config = Config::load("config.toml").expect("Failed on config");
        read_config.tunnels.insert(data.name.clone(), data);

        fs::write(
            "config.toml",
            toml::to_string(&read_config).expect("Failed to write config"),
        )?;

        Ok(())
    }
}
