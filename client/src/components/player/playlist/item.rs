use std::time::Duration;

use dioxus::prelude::*;

use crate::models::{
	player_context::{PlayerCommand, PlayerContext},
	playlist::PlaylistItem,
};
#[component]
pub fn DropMarker(drag_over: EventHandler<DragEvent>) -> Element
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
pub fn Item(
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
				div {
					class: "playlistItem loading {selected_class} {placeholder}",
					draggable: true,
					ondrag: on_drag,
					ondragover: on_over,
					div {
						class: "name",
						{key}
					}
					div {
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
				div {
					class: "playlistItem {selected_class} {placeholder}",
					draggable: true,
					ondrag: on_drag,
					ondragover: on_over,
					onclick: {
						move |_| {
							player_ctx.handle.send(PlayerCommand::SelectFile(key.clone()));
						}
					},
					div {
						class: "name",
						{display}
					}
					div {
						class: "info",
						"{to_duration_string(meta.duration)} seconds"
					}
				}
			}
		}
		PlaylistItem::NotFound { key } =>
		{
			rsx! {
				div {
					class: "playlistItem {selected_class} {placeholder}",
					draggable: true,
					ondrag: on_drag,
					ondragover: on_over,
					div {
						class: "name",
						{key}
					}
					div {
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
