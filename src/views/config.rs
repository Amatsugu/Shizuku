use dioxus::prelude::*;

use crate::models::config_file::ConfigContext;

#[component]
pub fn Config() -> Element {
	let cfg = use_context::<ConfigContext>();
	let dirs = match cfg {
		ConfigContext::Error(_) => Vec::default(),
		ConfigContext::Config(config_file) => config_file.media_dirs.clone(),
	};
	let dirs_text = use_signal(|| dirs.join("\n"));
	rsx! {
		label {
			"Media Directoies"
			textarea { {dirs_text()} }
		}
	}
}
