use dioxus::{html::HasFileData, prelude::*};

use crate::app::file_scanner::MEDIA_TYPES;

#[component]
pub fn FileDropZone(children: Element, ondrop: Callback<Vec<String>>) -> Element
{
	let mut is_dragging = use_signal(|| false);
	let on_drag_enter = move |_| {
		is_dragging.set(true);
	};

	let on_files_dropped = move |e: Event<DragData>| {
		e.prevent_default();
		info!("{} Files dropped", e.files().len());
		let files: Vec<String> = e
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
			.collect();
		if !files.is_empty()
		{
			ondrop.call(files);
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
			onmouseup: move |_|{ is_dragging.set(false); },
			div{
				class: "inner",
				{children}
			}
		}
	}
}
