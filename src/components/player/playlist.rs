use std::time::Duration;

use dioxus::{html::HasFileData, prelude::*};

use crate::{
	app::file_scanner::MEDIA_TYPES,
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
				handle_drop(e, player_ctx, &items, drag_from(), drag_to());
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

fn handle_drop(
	event: Event<DragData>,
	mut player_ctx: PlayerContext,
	items: &Vec<PlaylistItem>,
	from: Option<usize>,
	to: Option<usize>,
)
{
	let end = items.len();
	let mut to = to.unwrap_or(end);
	if let Some(from) = from
	{
		let mut items = items.clone();
		if from < to
		{
			to = to - 1;
		}
		let item = items.remove(from);
		items.insert(to, item);
		player_ctx.data.playlist.set(items);
	}
	info!("{} Files dropped", event.files().len());
	event
		.files()
		.iter()
		.filter(|f| {
			f.path()
				.extension()
				.and_then(|e| e.to_str())
				.map(|e| MEDIA_TYPES.contains(&e))
				.unwrap_or_default()
		})
		.filter_map(|f| f.path().to_str().map(|f| f.to_string()))
		.for_each(|f| {
			player_ctx.handle.send(PlayerCommand::AddFile(f, to));
		});
}

#[component]
fn DropMarker(drag_over: EventHandler<DragEvent>) -> Element
{
	rsx! {
		div {
			class: "dropMarker",
			ondragover: move |e: Event<DragData>|{
				e.prevent_default();
				e.stop_propagation();
				drag_over.call(e);
			}
		}
	}
}

#[component]
fn Item(
	item: PlaylistItem,
	selected: bool,
	placeholder: bool,
	drag_start: EventHandler<DragEvent>,
	drag_over: EventHandler<DragEvent>,
) -> Element
{
	let player_ctx = use_context::<PlayerContext>();
	let selected_class = if selected { "selected" } else { "" };
	let on_drag = move |e: Event<DragData>| {
		drag_start.call(e);
	};

	let on_over = move |e: Event<DragData>| {
		e.prevent_default();
		e.stop_propagation();
		drag_over.call(e);
		info!("Over");
	};

	let placeholder = if placeholder { "placeholder" } else { "" };
	match item
	{
		PlaylistItem::Unloaded { key } =>
		{
			rsx! {
				div{
					class: "playlistItem loading {selected_class} {placeholder}",
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
		PlaylistItem::Loaded { key, meta } =>
		{
			let display = key.clone();
			rsx! {
				div{
					class: "playlistItem {selected_class} {placeholder}",
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
		PlaylistItem::NotFound { key } =>
		{
			rsx! {
				div{
					class: "playlistItem {selected_class} {placeholder}",
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

fn to_duration_string(duration: Duration) -> String
{
	match duration
	{
		d if d.as_secs() < 60 => format!("0:{}", d.as_secs()),
		d if d.as_secs() < 60 * 60 =>
		{
			let s = d.as_secs();
			let m = s / 60;
			format!("{}:{}", m, s - (m * 60))
		}
		d if d.as_secs() < 60 * 60 * 60 =>
		{
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
