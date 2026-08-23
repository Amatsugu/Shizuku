use std::time::Duration;

use dioxus::{html::HasFileData, prelude::*};

use crate::models::toasts::{ToastCommand, ToastLevel, ToastsContext};

#[component]
pub fn FileDropZone(children: Element, ondrop: Callback<String>) -> Element
{
	let mut is_dragging = use_signal(|| false);
	let on_drag_enter = move |_| {
		is_dragging.set(true);
	};

	let toasts_ctx = use_context::<ToastsContext>();
	let on_files_dropped = move |e: Event<DragData>| {
		e.prevent_default();

		if let Some(first) = e.files().first()
		{
			match first.path().into_string()
			{
				Ok(path) => ondrop.call(path),
				Err(_) => toasts_ctx.handle.send(ToastCommand::PushWithDuration {
					title: "File drop failed".into(),
					message: None,
					level: ToastLevel::Error,
					duration: Duration::from_secs(5),
				}),
			}
		}
		is_dragging.set(false);
	};

	let drag_class = use_memo(move || match is_dragging()
	{
		true => "dragging",
		false => "",
	});

	rsx! {
		div{
			class: "fileDropZone {drag_class}",
			ondrop: on_files_dropped,
			ondragenter: on_drag_enter,
			ondragover: on_drag_enter,
			ondragstart: on_drag_enter,
			onmouseup: move |_|{ is_dragging.set(false); },
			input { type: "file" },
			div{
				class: "inner",
				{children}
			}
		}
	}
}
