use futures_util::stream::StreamExt;
use std::{env, fs, path::PathBuf, time::Duration};
use tokio::time::interval;

use dioxus::prelude::*;

use crate::{
	app::{mpv::Mpv, mpv_read::handle_mpv_read, playback_commands::handle_playback_commands},
	models::{
		player_context::{PlayerCommand, PlayerContext, PlayerData},
		playlist::PlaylistItem,
		toasts::{ToastCommand, ToastsContext},
	},
};

pub fn get_mpv_path(configured_path: Option<String>) -> Result<PathBuf, String> {
	if let Some(path) = configured_path
		&& !path.is_empty()
		&& fs::exists(path.clone()).map_err(|e| e.to_string())?
	{
		Ok(path.into())
	} else {
		if let Some(path) = env::var_os("PATH").and_then(|p| {
			env::split_paths(&p).find_map(|dir| {
				#[cfg(not(windows))]
				const PROGRAM: &str = "mpv";
				#[cfg(windows)]
				const PROGRAM: &str = "mpv.exe";
				let path = dir.join(PROGRAM);
				if path.is_file() { Some(path) } else { None }
			})
		}) {
			Ok(path)
		} else {
			Err("mpv not found in PATH".into())
		}
	}
}

pub fn init_player_context(mpv_path: Memo<Result<PathBuf, String>>) -> PlayerContext {
	let toasts_ctx = use_context::<ToastsContext>();
	let mut data = PlayerData {
		is_running: use_signal(|| true),
		playlist: use_signal(|| {
			vec![
				PlaylistItem::NotFound {
					key: "test1.mp4".into(),
				},
				PlaylistItem::NotFound {
					key: "test2.mp4".into(),
				},
				PlaylistItem::NotFound {
					key: "test3.mp4".into(),
				},
				PlaylistItem::NotFound {
					key: "test4.mp4".into(),
				},
				PlaylistItem::NotFound {
					key: "test5.mp4".into(),
				},
			]
		}),
		users: use_signal(Vec::new),
		selected_file: use_signal(|| None),
	};
	let handle = use_coroutine(
		move |mut rx: dioxus::prelude::UnboundedReceiver<PlayerCommand>| {
			let mpv_path = mpv_path.cloned();
			async move {
				let mpv_path = match mpv_path {
					Ok(path) => path,
					Err(err) => {
						toasts_ctx.handle.send(ToastCommand::Push {
							title: "MPV path not set".into(),
							message: Some(err.to_string()),
							level: crate::models::toasts::ToastLevel::Error,
						});
						return;
					}
				};
				let mut mpv = match Mpv::start(mpv_path).await {
					Ok(mpv) => mpv,
					Err(err) => {
						toasts_ctx.handle.send(ToastCommand::Push {
							title: "Could not start mpv".into(),
							message: Some(err.to_string()),
							level: crate::models::toasts::ToastLevel::Error,
						});
						return;
					}
				};
				let mut health_check = interval(Duration::from_secs(1));
				health_check.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

				loop {
					tokio::select! {
						Some(cmd) = rx.next() => {
							handle_playback_commands(cmd, &mut mpv, toasts_ctx, data).await
						}
						result = mpv.read.next_line() =>{
							handle_mpv_read(result);
						}
						_ = health_check.tick() => {
							if !mpv.is_running()
							{
								info!("No longer running");
								data.is_running.set(false);
								break;
							}
						}
					}
				}
			}
		},
	);
	PlayerContext { handle, data }
}
