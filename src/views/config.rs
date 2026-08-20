use dioxus::prelude::*;

use crate::{CONFIG_PATH, components::basic::Button, models::config_file::ConfigContext};

#[component]
pub fn Config() -> Element {
	let mut ctx = use_context::<ConfigContext>();
	let mut cfg = ctx.config;
	let dirs_text = use_memo(move || cfg.cloned().media_dirs.join("\n"));
	rsx! {
		div {
			class: "configForm",
			label {
				"Media Directoies"
				textarea {
					oninput: move |e|{
						let mut cur_cfg = cfg.cloned();
						cur_cfg.media_dirs = e.value().lines().map(|e|e.to_string()).collect();
						cfg.set(cur_cfg)
					},
					value : dirs_text()
				}
			}

			div {
				class: "buttonRow",
				Button {
					onclick: move |_|{
						_ = cfg.cloned().save_config(CONFIG_PATH);
					},
					"Save"
				}

				Button {
					onclick: move |_|{
						ctx.reload_config();
					},
					"Reload Config"
				}
			}
		}
	}
}
