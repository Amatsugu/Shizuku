use std::time::Duration;

use dioxus::prelude::*;

use crate::{
	CONFIG_PATH,
	components::{
		ServersEditor,
		basic::{Button, Input, InputValue},
	},
	models::{
		config_file::ConfigContext,
		toasts::{ToastCommand, ToastLevel, ToastsContext},
	},
};

#[component]
pub fn Config() -> Element
{
	let mut ctx = use_context::<ConfigContext>();
	let mut cfg = ctx.config;
	let toast_ctx = use_context::<ToastsContext>();
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
					}
					else
					{
						c.mpv_path = Some(val);
					}
				}
			}
			h2 { "Media Files" }
			label {
				"Media Directories"
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
						if let Err(err) = cfg.cloned().save_config(CONFIG_PATH){
							toast_ctx.handle.send(ToastCommand::Push { title: "Failed to Save Config".into(), message: Some(err), level: ToastLevel::Error });
						}else{
							toast_ctx.handle.send(ToastCommand::PushWithDuration { title: "Config saved".into(), message: None, level: ToastLevel::Info, duration: Duration::from_secs(5) });
						}
					},
					"Save"
				}

				Button {
					onclick: move |_|{
						ctx.reload_config();
						toast_ctx.handle.send(ToastCommand::PushWithDuration { title: "Config reloaded".into(), message: None, level: ToastLevel::Info, duration: Duration::from_secs(5) });
					},
					"Reload Config"
				}
			}
		}
	}
}
