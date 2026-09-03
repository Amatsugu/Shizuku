use dioxus::{
	desktop::{Config, LogicalSize, WindowBuilder},
	prelude::*,
};

use crate::{components::basic::Button, models::config_file::ConfigContext, route::Route};
mod app;
mod components;
mod layouts;
mod models;
mod route;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const BASE_CSS: Asset = asset!("/assets/base.scss");
pub const CONFIG_PATH: &str = "config.toml";

fn main() {
	let window = WindowBuilder::new()
		.with_title("Shizuku")
		.with_inner_size(LogicalSize::new(400.0, 400.0));
	LaunchBuilder::desktop()
		.with_cfg(Config::default().with_window(window))
		.launch(App);
}

#[component]
fn App() -> Element {
	let mut cfg = use_context_provider(ConfigContext::load_or_create_config_file);

	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: BASE_CSS }

		match cfg.load_state.cloned() {
			models::config_file::ConfigLoadState::Failed(msg) => rsx!{
				h1 { "Config Error" }
				p { {msg} }
				Button {
					onclick: move |_|{
						cfg.reload_config();
					},
					"Reload Config"
				}
			},
			models::config_file::ConfigLoadState::Loaded => rsx! {
				Router::<Route> {}
			},
		}
	}
}
