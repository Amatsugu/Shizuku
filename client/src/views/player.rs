use std::time::Duration;

use dioxus::prelude::*;

use crate::{
	app::{
		file_scanner::scan_dirs,
		playback::{get_mpv_path, init_player_context},
	},
	components::player::{PlayerControls, Playlist, UserList},
	models::{
		config_file::{ConfigContext, ConfigFile},
		player_context::PlayerData,
		playlist::{MediaMetadata, PlaylistItem},
	},
	route::Route,
};

#[component]
pub fn Player() -> Element {
	let config = use_context::<ConfigContext>().config;
	let mpv_path = use_memo(move || get_mpv_path(config().mpv_path));

	let player_ctx = init_player_context(
		mpv_path,
		config.cloned().username.unwrap_or("Unknown".into()),
	);
	let player_ctx = use_context_provider(|| player_ctx);
	init_file_scan(config, player_ctx.data);

	use_effect(move || {
		if !player_ctx.data.is_running.cloned() {
			navigator().push(Route::Home {});
		}
	});

	rsx! {
		div{
			id: "player",
			PlayerControls{}
			UserList{}
			Playlist{}
		}
	}
}

fn init_file_scan(config: Signal<ConfigFile>, mut player_data: PlayerData) {
	let files = use_resource(use_reactive!(|(config)| async move {
		scan_dirs(config.cloned().media_dirs).await
	}));

	use_effect(move || {
		let Some(files) = files() else {
			return;
		};
		let mut playlist = player_data.playlist.cloned();
		for itm in &mut playlist {
			if let PlaylistItem::Unloaded { key } = itm {
				match files.iter().find(|dir| {
					dir.file_name()
						.map(|f| f.eq_ignore_ascii_case(&key))
						.unwrap_or_default()
				}) {
					Some(path) => match MediaMetadata::probe_metadata(path) {
						Ok(meta) => {
							*itm = PlaylistItem::Loaded {
								key: key.clone(),
								meta,
							}
						}
						Err(err) => {
							*itm = PlaylistItem::Loaded {
								key: key.clone(),
								meta: MediaMetadata {
									duration: Duration::from_secs(0),
									path: path.clone(),
								},
							};
							error!("Failed to probe duration: {}", err);
						}
					},
					None => *itm = PlaylistItem::NotFound { key: key.clone() },
				}
			}
		}
		player_data.playlist.set(playlist);
	});
}
