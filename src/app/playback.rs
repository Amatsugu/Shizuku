use futures_util::stream::StreamExt;
use std::{path::PathBuf, time::Duration};
use tokio::time::interval;

use dioxus::prelude::*;

use crate::{
	app::mpv::Mpv,
	models::{
		mpv::{commands::MpvCommand, responses::MpvMessage},
		player_context::{PlayerCommand, PlayerContext},
		toasts::{ToastCommand, ToastLevel, ToastsContext},
	},
};

pub fn start_mpv(mpv_path: Memo<Result<PathBuf, String>>) -> PlayerContext
{
	let toasts_ctx = use_context::<ToastsContext>();
	let mut is_running = use_signal(|| true);
	let handle = use_coroutine(move |mut rx: dioxus::prelude::UnboundedReceiver<PlayerCommand>| {
		let mpv_path = mpv_path.cloned();
		async move {
			let mpv_path = match mpv_path
			{
				Ok(path) => path,
				Err(err) =>
				{
					toasts_ctx.handle.send(ToastCommand::Push {
						title: "MPV path not set".into(),
						message: Some(err.to_string()),
						level: crate::models::toasts::ToastLevel::Error,
					});
					return;
				}
			};
			let mut mpv = match Mpv::start(mpv_path).await
			{
				Ok(mpv) => mpv,
				Err(err) =>
				{
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

			loop
			{
				tokio::select! {
					Some(cmd) = rx.next() => {
						let result = match cmd {
							PlayerCommand::OpenFile(path) => mpv.open_file(path).await,
							PlayerCommand::Play => mpv.send_command(MpvCommand::play()).await,
							PlayerCommand::Pause => mpv.send_command(MpvCommand::pause()).await,
							PlayerCommand::Seek(time) => mpv.send_command(MpvCommand::seek(time)).await,
						};
						if let Err(err) = result {
							toasts_ctx.handle.send(ToastCommand::PushWithDuration { title: "Failed to communicate with mpv".into(), message: Some(err.to_string()), level: ToastLevel::Error, duration: Duration::from_secs(5) });
						}
					}
					result = mpv.read.next_line() =>{
						match result {
							Ok(Some(line)) => {
								let _reply = MpvMessage::parse(line);
							},
							Ok(None) => {}
							Err(err) => {
								error!("Failed to read mpv stream: {}", err);
							},
						}
					}
					_ = health_check.tick() => {
						if !mpv.is_running()
						{
							info!("No longer running");
							is_running.set(false);
							break;
						}
					}
				}
			}
		}
	});
	PlayerContext { handle, is_running }
}
