use std::path::PathBuf;

use dioxus::prelude::*;
use jwalk::WalkDir;

use crate::{models::config_file::ConfigContext, route::Route};

#[component]
pub fn Player() -> Element {
	let config = use_context::<ConfigContext>().config;
	let dirs = use_resource(use_reactive!(|(config)| async move {
		scan_dirs(config.cloned().media_dirs).await
	}));
	match dirs() {
		Some(dirs) => rsx! {
			p {"Found {dirs.len()} files" }
			Link{
				to: Route::Home {  },
				"Back"
			}
		},
		None => rsx! {"Scanning dirs"},
	}
}

const MEDIA_TYPES: &[&str] = &["mp4", "mkv", "mov", "webm", "avi"];

async fn scan_dirs(media_dirs: Vec<String>) -> Vec<PathBuf> {
	tokio::task::spawn_blocking(move || {
		media_dirs
			.iter()
			.flat_map(|media_dir| {
				WalkDir::new(media_dir).into_iter().filter_map(|e| {
					if let Ok(entry) = e
						&& entry.file_type.is_file()
						&& let Some(ext) = entry.path().extension()
						&& let Some(ext) = ext.to_str()
						&& MEDIA_TYPES.contains(&ext)
					{
						Some(entry.path())
					} else {
						None
					}
				})
			})
			.collect()
	})
	.await
	.unwrap_or_default()
}
