use std::{env, fs, path::PathBuf};

use dioxus::prelude::*;
use jwalk::WalkDir;

use crate::{
	app::playback::start_mpv,
	components::FileDropZone,
	models::{
		config_file::ConfigContext,
		player_context::PlayerCommand,
		toasts::{ToastCommand, ToastLevel, ToastsContext},
	},
	route::Route,
};

#[component]
pub fn Player() -> Element
{
	let config = use_context::<ConfigContext>().config;
	let mpv_path = use_memo(move || get_mpv_path(config().mpv_path));
	let toast_ctx = use_context::<ToastsContext>();

	let player_ctx = start_mpv(mpv_path);
	let player_ctx = use_context_provider(|| player_ctx);
	use_effect(move || {
		if !player_ctx.is_running.cloned()
		{
			navigator().push(Route::Home {});
		}
	});

	let dirs = use_resource(use_reactive!(|(config)| async move {
		scan_dirs(config.cloned().media_dirs).await
	}));
	match dirs()
	{
		Some(dirs) => rsx! {
			p {"Found {dirs.len()} files" }
			Link{
				to: Route::Home {  },
				"Back"
			}

			FileDropZone{
				ondrop: move |path|{
					player_ctx.handle.send(PlayerCommand::OpenFile(path));
				},
				div{
					"Drop files here"
				}
			}
		},
		None => rsx! {"Scanning dirs"},
	}
}

const MEDIA_TYPES: &[&str] = &["mp4", "mkv", "mov", "webm", "avi"];

async fn scan_dirs(media_dirs: Vec<String>) -> Vec<PathBuf>
{
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
					}
					else
					{
						None
					}
				})
			})
			.collect()
	})
	.await
	.unwrap_or_default()
}

fn get_mpv_path(configured_path: Option<String>) -> Result<PathBuf, String>
{
	if let Some(path) = configured_path
		&& !path.is_empty()
		&& fs::exists(path.clone()).map_err(|e| e.to_string())?
	{
		Ok(path.into())
	}
	else
	{
		if let Some(path) = env::var_os("PATH").and_then(|p| {
			env::split_paths(&p).find_map(|dir| {
				#[cfg(not(windows))]
				const PROGRAM: &str = "mpv";
				#[cfg(windows)]
				const PROGRAM: &str = "mpv.exe";
				let path = dir.join(PROGRAM);
				if path.is_file() { Some(path) } else { None }
			})
		})
		{
			Ok(path)
		}
		else
		{
			Err("mpv not found in PATH".into())
		}
	}
}
