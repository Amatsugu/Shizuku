use dioxus::prelude::*;

use crate::{
	CONFIG_PATH,
	components::{
		ServersEditor,
		basic::{Button, Input, InputValue},
	},
	models::config_file::ConfigContext,
};

#[component]
pub fn Config() -> Element {
	let mut ctx = use_context::<ConfigContext>();
	let mut cfg = ctx.config;
	let dirs_text = use_memo(move || cfg.cloned().media_dirs.join("\n"));
	let mpv_path = use_memo(move || cfg.cloned().mpv_path.unwrap_or_default());
	rsx! {
		div {
			class: "configForm",
			h2 { "Media Player" }
			Input {
				label: "MPV path",
				name: "mpv_path",
				value: InputValue::Const(mpv_path.cloned()),
				oninput: move |e : Event<FormData>|{
					let mut c = cfg.cloned();
					let val = e.value();
					if val.is_empty() {
						c.mpv_path = None;
					}else
					{
						c.mpv_path = Some(val);
					}
				}
			}
			h2 { "Media Files" }
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
			ServersEditor {}
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
