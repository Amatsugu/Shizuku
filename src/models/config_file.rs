use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ConfigFile
{
	pub media_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ConfigContext
{
	Error(String),
	Config(ConfigFile),
}

impl ConfigContext
{
	pub fn from_file_path<P: AsRef<Path>>(path: P) -> Self
	{
		match fs::read_to_string(path)
		{
			Ok(data) => match toml::from_str::<ConfigFile>(&data)
			{
				Ok(cfg) => ConfigContext::Config(cfg),
				Err(err) => ConfigContext::Error(format!("Failed to parse config file: {}", err)),
			},
			Err(err) => ConfigContext::Error(format!("Failed to load config file: {}", err)),
		}
	}
}
