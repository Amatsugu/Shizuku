use dioxus::prelude::*;

use crate::models::config_file::ConfigContext;

#[component]
pub fn Config() -> Element {
	let cfg = use_context::<ConfigContext>().config;
	let dirs_text = use_memo(move || cfg.cloned().media_dirs.join("\n"));
	rsx! {
		label {
			"Media Directoies"
			textarea { {dirs_text()} }
		}
	}
}
