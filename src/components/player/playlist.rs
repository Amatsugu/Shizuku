use dioxus::prelude::*;

use crate::{
	components::FileDropZone,
	models::{
		player_context::{PlayerCommand, PlayerContext},
		playlist::PlaylistItem,
	},
};

#[component]
pub fn Playlist() -> Element
{
	let player_ctx = use_context::<PlayerContext>();
	rsx! {
		div{
			id: "playlist",
			FileDropZone{
				ondrop: move |files|{
					for path in files {
						player_ctx.handle.send(PlayerCommand::AddFile(path));
					}
				},
				ItemList { items: player_ctx.data.playlist.cloned() }
			}
		}
	}
}

#[component]
fn ItemList(items: Vec<PlaylistItem>) -> Element
{
	rsx! {
		div{
			class: "list",
			if items.is_empty(){
				span { class: "palcehodler", "Drop files here to add to playlist." }
			}
			for item in items {
				Item { item }
			}
		}
	}
}

#[component]
fn Item(item: PlaylistItem) -> Element
{
	let player_ctx = use_context::<PlayerContext>();
	match item
	{
		PlaylistItem::Unloaded { key } =>
		{
			rsx! {
				div{
					class: "playlistItem loading",
					div{
						class: "name",
						{key}
					}
					div{
						class: "info",
						"Loading.."
					}
				}
			}
		}
		PlaylistItem::Loaded { key, meta } =>
		{
			let display = key.clone();
			rsx! {
				div{
					class: "playlistItem",
					onclick: {
						move |_| {
							player_ctx.handle.send(PlayerCommand::SelectFile(key.clone()));
						}
					},
					div{
						class: "name",
						{display}
					}
					div{
						class: "info",
						"{meta.duration.as_secs()} seconds"
					}
				}
			}
		}
		PlaylistItem::NotFound { key } =>
		{
			rsx! {
				div{
					class: "playlistItem",
					div{
						class: "name",
						{key}
					}
					div{
						class: "info",
						"Not Found"
					}
				}
			}
		}
	}
}
