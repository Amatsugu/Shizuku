use std::path::Path;

use serde::Serialize;

#[derive(Default, Serialize)]
pub struct MpvCommand
{
	pub command: Vec<String>,
	pub request_id: Option<u64>,
}

impl MpvCommand
{
	pub fn load_file<T: AsRef<Path>>(path: T) -> Option<Self>
	{
		let path = path.as_ref().to_str()?;
		Some(MpvCommand {
			command: vec!["loadfile".into(), path.to_string()],
			..Default::default()
		})
	}

	pub fn pause() -> Self
	{
		MpvCommand {
			command: vec!["set_property".into(), "pause".into(), "true".into()],
			..Default::default()
		}
	}

	pub fn play() -> Self
	{
		MpvCommand {
			command: vec!["set_property".into(), "pause".into(), "false".into()],
			..Default::default()
		}
	}

	pub fn seek(time: u64) -> Self
	{
		MpvCommand {
			command: vec!["seek".into(), time.to_string()],
			..Default::default()
		}
	}

	pub fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		serde_json::to_vec(self).map_err(|e| e.to_string())
	}
}
