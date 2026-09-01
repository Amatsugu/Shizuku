use std::time::Duration;

use dioxus::prelude::*;

use crate::{
	components::FileDropZone,
	models::{
		player_context::{PlayerCommand, PlayerContext},
		playlist::PlaylistItem,
	},
};

#[component]
pub fn Playlist() -> Element {
	let player_ctx = use_context::<PlayerContext>();
	rsx! {
		div{
			id: "playlist",
			// FileDropZone{
			// 	ondrop: move |files|{
			// 		for path in files {
			// 			player_ctx.handle.send(PlayerCommand::AddFile(path));
			// 		}
			// 	},
			// }
			ItemList { items: player_ctx.data.playlist.cloned() }
		}
	}
}

#[component]
fn ItemList(items: Vec<PlaylistItem>) -> Element {
	let player_ctx = use_context::<PlayerContext>();
	let end = items.len();
	let mut drag_from = use_signal(|| Option::<usize>::None);
	let mut drag_to = use_signal(|| items.len());
	let on_drag_end = move |e: Event<DragData>| {
		e.prevent_default();
		info!("end");
		drag_from.set(None);
		drag_to.set(end);
	};
	rsx! {
		div{
			class: "list",

			ondrop: move |e: Event<DragData>|{
				e.prevent_default();
				info!("drop");
				if let Some(from) = drag_from(){
					info!("From: {}, to: {}", from, drag_to());
				}
			},
			ondragover: move |e: Event<DragData>|{
				e.prevent_default();
				drag_to.set(end);
			},
			ondragend: on_drag_end,
			ondragexit: on_drag_end,
			if items.is_empty(){
				span { class: "palcehodler", "Drop files here to add to playlist." }
			}
			div { "f:{drag_from().unwrap_or_default()} t:{drag_to()}" }
			for (idx, item) in items.iter().enumerate() {
				Item
				{
					item: item.clone(),
					selected: player_ctx.data.selected_file.cloned().map(|s| s == idx).unwrap_or_default(),
					drag_start: move |_|{
						drag_from.set(Some(idx));
					},
					drag_over: move |_|{
						drag_to.set(idx);
					}
				}
			}
		}
	}
}

#[component]
fn Item(
	item: PlaylistItem,
	selected: bool,
	drag_start: EventHandler<DragEvent>,
	drag_over: EventHandler<DragEvent>,
) -> Element {
	let player_ctx = use_context::<PlayerContext>();
	let selected_class = if selected { "selected" } else { "" };
	let on_drag = move |e: Event<DragData>| {
		drag_start.call(e);
	};

	let on_over = move |e: Event<DragData>| {
		e.prevent_default();
		drag_over.call(e);
	};
	match item {
		PlaylistItem::Unloaded { key } => {
			rsx! {
				div{
					class: "playlistItem loading {selected_class}",
					draggable: true,
					ondrag: on_drag,
					ondragstart: on_drag,
					ondragover: on_over,
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
		PlaylistItem::Loaded { key, meta } => {
			let display = key.clone();
			rsx! {
				div{
					class: "playlistItem {selected_class}",
					draggable: true,
					ondrag: on_drag,
					ondragstart: on_drag,
					ondragover: on_over,
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
						"{to_duration_string(meta.duration)} seconds"
					}
				}
			}
		}
		PlaylistItem::NotFound { key } => {
			rsx! {
				div{
					class: "playlistItem {selected_class}",
					draggable: true,
					ondrag: on_drag,
					ondragstart: on_drag,
					ondragover: on_over,
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

fn to_duration_string(duration: Duration) -> String {
	match duration {
		d if d.as_secs() < 60 => format!("0:{}", d.as_secs()),
		d if d.as_secs() < 60 * 60 => {
			let s = d.as_secs();
			let m = s / 60;
			format!("{}:{}", m, s - (m * 60))
		}
		d if d.as_secs() < 60 * 60 * 60 => {
			let mut s = d.as_secs();
			let h = s / (60 * 60);
			s -= h * 60 * 60;
			let m = s / 60;
			s -= m * 60;
			format!("{}:{}:{}", h, m, s)
		}
		_ => "".into(),
	}
}
