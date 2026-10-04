use std::{
	path::{Path, PathBuf},
	process::Command,
	time::Duration,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PlaylistItem
{
	Unloaded
	{
		key: String
	},
	Loaded
	{
		key: String, meta: MediaMetadata
	},
	NotFound
	{
		key: String
	},
}

impl PlaylistItem
{
	pub fn key(&self) -> &String
	{
		match self
		{
			PlaylistItem::Unloaded { key } => key,
			PlaylistItem::Loaded { key, .. } => key,
			PlaylistItem::NotFound { key } => key,
		}
	}

	pub fn key_matches(&self, key: &String) -> bool
	{
		self.key() == key
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct MediaMetadata
{
	pub duration: Duration,
	pub path: PathBuf,
}

impl MediaMetadata
{
	pub fn probe_metadata(path: impl AsRef<Path>) -> Result<MediaMetadata, String>
	{
		let file = path.as_ref().to_str().unwrap_or_default().to_string();
		let duration = Command::new("ffprobe")
			.arg("-v")
			.arg("error")
			.arg("-show_entries")
			.arg("format=duration")
			.arg("-of")
			.arg("default=noprint_wrappers=1:nokey=1")
			.arg(file)
			.output()
			.map_err(|e| e.to_string())?;
		let duration_str = str::from_utf8(&duration.stdout).map_err(|e| e.to_string())?.trim();
		dbg!(duration_str);
		let duration = duration_str
			.parse::<f32>()
			.map(|d| Duration::from_secs_f32(d))
			.map_err(|e| e.to_string());
		Ok(MediaMetadata {
			duration: duration?,
			path: path.as_ref().to_path_buf(),
		})
	}
}
