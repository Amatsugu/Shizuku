use std::{path::Path, time::Duration};

use dioxus::{
	logger::tracing::info,
	signals::{ReadableExt, Signal, WritableExt},
};

use crate::{
	app::mpv::Mpv,
	models::{
		mpv::commands::MpvCommand,
		player_context::{PlayerCommand, PlayerData},
		playlist::PlaylistItem,
		toasts::{ToastCommand, ToastLevel, ToastsContext},
	},
};

pub async fn handle_playback_commands(
	cmd: PlayerCommand,
	mpv: &mut Mpv,
	toasts_ctx: ToastsContext,
	player_data: PlayerData,
)
{
	let result = match cmd
	{
		PlayerCommand::AddFile(path) =>
		{
			handle_file_add(path, mpv, player_data.playlist, player_data.selected_file).await
		}
		PlayerCommand::SelectFile(key) =>
		{
			handle_select_file(key, mpv, player_data.playlist, player_data.selected_file).await
		}
		PlayerCommand::Play => mpv.send_command(MpvCommand::play()).await,
		PlayerCommand::Pause => mpv.send_command(MpvCommand::pause()).await,
		PlayerCommand::Seek(time) => mpv.send_command(MpvCommand::seek(time)).await,
	};
	if let Err(err) = result
	{
		toasts_ctx.handle.send(ToastCommand::PushWithDuration {
			title: "Failed to communicate with mpv".into(),
			message: Some(err.to_string()),
			level: ToastLevel::Error,
			duration: Duration::from_secs(5),
		});
	}
}

async fn handle_select_file(
	key: String,
	mpv: &mut Mpv,
	playlist: Signal<Vec<PlaylistItem>>,
	mut selected: Signal<Option<usize>>,
) -> Result<(), String>
{
	info!("Select: {}", key);
	let list = playlist.cloned();
	let Some((idx, playlist_item)) = list.iter().enumerate().find(|(_, itm)| itm.key_matches(&key))
	else
	{
		return Err("Failed to find matching playlist item".into());
	};
	if let PlaylistItem::Loaded { meta, .. } = playlist_item
	{
		mpv.open_file(meta.path.as_path()).await?;
		selected.set(Some(idx));
		Ok(())
	}
	else
	{
		Err("Playlist item not found on local file system".into())
	}
}

async fn handle_file_add(
	path: String,
	mpv: &mut Mpv,
	mut playlist: Signal<Vec<PlaylistItem>>,
	mut selected: Signal<Option<usize>>,
) -> Result<(), String>
{
	let path = Path::new(&path);

	if !path.exists()
	{
		return Err("File does not exist".into());
	}
	let Some(filename) = path.file_name().and_then(|f| f.to_str().map(|f| f.to_string()))
	else
	{
		return Err("Failed to extract filename".into());
	};

	if !playlist.read().iter().any(|l| l.key_matches(&filename))
	{
		let idx = playlist.read().len();
		playlist.write().push(PlaylistItem::Unloaded { key: filename });
		if selected.read().is_none()
		{
			mpv.open_file(path).await?;
			selected.set(Some(idx));
		}
	}

	Ok(())
}
