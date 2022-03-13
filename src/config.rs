#![allow(clippy::module_name_repetitions)]

use clap::Parser;
use serde::{de::DeserializeOwned, Deserialize};
use std::{path::PathBuf, net::SocketAddr};
use url::Url;

#[derive(Parser)]
#[clap(version = clap::crate_version!(), author = clap::crate_authors!())]
pub struct Args {
    #[clap(short, long, parse(try_from_str = from_toml_path))]
    pub config: Config,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub listen_address: SocketAddr,
    pub state_directory: PathBuf,
    pub gitlab: GitlabConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GitlabConfig {
    pub uri: Url,
    pub admin_token: String,
}

pub fn from_toml_path<T: DeserializeOwned>(path: &str) -> Result<T, std::io::Error> {
    let contents = std::fs::read(path)?;
    toml::from_slice(&contents).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
