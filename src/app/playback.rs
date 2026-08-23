use futures_util::stream::StreamExt;
use std::{io::Write, path::PathBuf};

use dioxus::{
	hooks::{use_context, use_coroutine},
	signals::{Memo, ReadableExt},
};

use crate::{
	app::mpv::Mpv,
	models::{
		player_context::{PlayerCommands, PlayerContext},
		toasts::{ToastCommand, ToastsContext},
	},
};

pub fn start_mpv(mpv_path: Memo<Result<PathBuf, String>>) -> PlayerContext
{
	let toasts_ctx = use_context::<ToastsContext>();
	let handle = use_coroutine(move |mut rx: dioxus::prelude::UnboundedReceiver<PlayerCommands>| {
		let mpv = mpv_path.cloned().and_then(Mpv::start_mpv);
		async move {
			let mut mpv = match mpv
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

			_ = mpv
				.write
				.write_all(br#"{ "command": ["loadfile", "Z:/VLive/robo4th.mp4"] }"#);
			_ = mpv.write.write_all(b"\n");

			loop
			{
				tokio::select! {
					Some(cmd) = rx.next() =>{
						match cmd {
							PlayerCommands::OpenFile(_) => todo!(),
							PlayerCommands::Play => todo!(),
							PlayerCommands::Pause => todo!(),
							PlayerCommands::Seek(_) => todo!(),
						}
					}
				}
			}
		}
	});
	PlayerContext { handle }
}
