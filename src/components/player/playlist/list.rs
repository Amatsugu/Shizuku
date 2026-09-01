use dioxus::prelude::*;

use crate::{
	components::player::playlist::item::{DropMarker, Item},
	models::{player_context::PlayerContext, playlist::PlaylistItem},
};

#[component]
pub fn Playlist() -> Element
{
	let player_ctx = use_context::<PlayerContext>();
	rsx! {
		div{
			id: "playlist",
			ItemList { items: player_ctx.data.playlist.cloned() }
		}
	}
}

#[component]
fn ItemList(items: Vec<PlaylistItem>) -> Element
{
	let player_ctx = use_context::<PlayerContext>();
	let end = items.len();
	let mut drag_from = use_signal(|| Option::<usize>::None);
	let mut drag_to = use_signal(|| Option::<usize>::None);
	let on_drag_end = move |e: Event<DragData>| {
		e.prevent_default();
		info!("end");
	};
	rsx! {
		div{
			class: "list",

			ondrop: move |e: Event<DragData>|{
				e.prevent_default();
				info!("drop");
				let to = drag_to().unwrap_or(items.len());
				player_ctx.handle_reorder(drag_from(), to, &items);
				player_ctx.handle_file_drop(e, to);
				drag_from.set(None);
				drag_to.set(None);
			},
			ondragover: move |e: Event<DragData>|{
				e.prevent_default();
				drag_to.set(Some(end));
			},
			ondragend: on_drag_end,
			ondragexit: on_drag_end,
			if items.is_empty(){
				span { class: "palcehodler", "Drop files here to add to playlist." }
			}
			for (idx, item) in items.iter().enumerate() {
				if let Some(to) = drag_to() && to == idx{
					DropMarker {
						drag_over: move |_|{
							drag_to.set(Some(idx));
						}
					}
				}
				Item
				{
					item: item.clone(),
					placeholder: drag_from().map(|f| f == idx).unwrap_or_default(),
					selected: player_ctx.data.selected_file.cloned().map(|s| s == idx).unwrap_or_default(),
					drag_start: move |_|{
						drag_from.set(Some(idx));
					},
					drag_over: move |_|{
						drag_to.set(Some(idx));
					}
				}
			}
			if let Some(to) = drag_to() && to == end{
				DropMarker {
					drag_over: move |_|{
						drag_to.set(Some(end));
					}
				}
			}
		}
	}
}
