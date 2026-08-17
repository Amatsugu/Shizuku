use dioxus::prelude::*;
use jwalk::WalkDir;

use crate::models::config_file::ConfigContext;

#[component]
pub fn Player() -> Element
{
	let config = use_context::<ConfigContext>();
	let dirs = match config
	{
		ConfigContext::Error(_) => Vec::default(),
		ConfigContext::Config(config_file) => config_file.media_dirs.clone(),
	};
	let dirs = use_resource(use_reactive!(|(dirs)| async move { scan_dirs(dirs).await }));
	match dirs()
	{
		Some(dirs) => rsx! {"Found {dirs.len()} files"},
		None => rsx! {"Scanning dirs"},
	}
}

const MEDIA_TYPES: &[&str] = &["mp4", "mkv", "mov", "webm", "avi"];

async fn scan_dirs(media_dirs: Vec<String>) -> Vec<String>
{
	let files = media_dirs
		.iter()
		.flat_map(|media_dir| {
			WalkDir::new(media_dir).into_iter().filter_map(|e| {
				if let Ok(entry) = e
					&& entry.file_type.is_file()
					&& let Some(ext) = entry.path().extension()
					&& let Some(ext) = ext.to_str()
					&& MEDIA_TYPES.contains(&ext)
				{
					entry.path().into_string().ok()
				}
				else
				{
					None
				}
			})
		})
		.collect::<Vec<String>>();
	files
}
