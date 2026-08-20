use dioxus::prelude::*;

use crate::{components::basic::Button, models::config_file::ConfigContext, route::Route};
mod app;
mod components;
mod layouts;
mod models;
mod route;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.scss");
pub const CONFIG_PATH: &str = "config.toml";

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	let mut cfg = use_context_provider(ConfigContext::load_or_create_config_file);

	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }

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
			models::config_file::ConfigLoadState::Loaded => rsx! {Router::<Route> {}},
		}
	}
}
