use std::{fs, path::Path};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::CONFIG_PATH;

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct ConfigFile
{
	pub media_dirs: Vec<String>,
	pub mpv_path: Option<String>,
	pub last_server: Option<String>,
	pub servers: Vec<Server>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Server
{
	pub name: String,
	pub host: String,
	pub password: Option<String>,
}

impl Default for Server
{
	fn default() -> Self
	{
		Self {
			name: "New Server".into(),
			host: Default::default(),
			password: Default::default(),
		}
	}
}

impl ConfigFile
{
	fn load_config<P: AsRef<Path>>(path: P) -> Result<ConfigFile, String>
	{
		let data = fs::read_to_string(path).map_err(|e| format!("Failed to load config file: {}", e))?;
		toml::from_str::<ConfigFile>(&data).map_err(|e| format!("Failed to parse config file: {}", e))
	}

	pub fn save_config<P: AsRef<Path>>(&self, path: P) -> Result<(), String>
	{
		let data = toml::to_string(self).map_err(|e| e.to_string())?;
		fs::write(path, data).map_err(|e| e.to_string())?;
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigContext
{
	pub config: Signal<ConfigFile>,
	pub load_state: Signal<ConfigLoadState>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigLoadState
{
	Failed(String),
	Loaded,
}

impl ConfigContext
{
	pub fn load_or_create_config_file() -> Self
	{
		match Self::load_or_create_config()
		{
			Ok(cfg) => ConfigContext {
				config: Signal::new(cfg),
				load_state: Signal::new(ConfigLoadState::Loaded),
			},
			Err(err) => Self::from_err_string(err),
		}
	}

	fn load_or_create_config() -> Result<ConfigFile, String>
	{
		if !fs::exists(CONFIG_PATH).map_err(|err| format!("Failed to read config path: {}", err))?
		{
			fs::write(
				CONFIG_PATH,
				toml::to_string(&ConfigFile::default()).map_err(|e| e.to_string())?,
			)
			.map_err(|err| format!("Failed to create default config file: {}", err))?;
		}
		ConfigFile::load_config(CONFIG_PATH)
	}

	// pub fn from_file_path<P: AsRef<Path>>(path: P) -> Self {
	// 	match ConfigFile::load_config(path) {
	// 		Ok(cfg) => Self {
	// 			config: Signal::new(cfg),
	// 			load_state: Signal::new(ConfigLoadState::Loaded),
	// 		},
	// 		Err(err) => Self::from_err_string(err),
	// 	}
	// }

	pub fn from_err_string(err: String) -> Self
	{
		Self {
			config: Default::default(),
			load_state: Signal::new(ConfigLoadState::Failed(err)),
		}
	}

	pub fn reload_config(&mut self)
	{
		match Self::load_or_create_config()
		{
			Ok(cfg) =>
			{
				self.config.set(cfg);
				self.load_state.set(ConfigLoadState::Loaded);
			}
			Err(err) => self.load_state.set(ConfigLoadState::Failed(err)),
		}
	}
}
