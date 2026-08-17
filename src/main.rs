use std::fs;

use dioxus::prelude::*;

use crate::{models::config_file::ConfigContext, route::Route};
mod app;
mod components;
mod layouts;
mod models;
mod route;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.scss");
const CONFIG_PATH: &str = "config.toml";

fn main()
{
	dioxus::launch(App);
}

#[component]
fn App() -> Element
{
	let cfg = use_context_provider(load_or_create_config_file);

	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }

		match cfg {
			ConfigContext::Error(msg) => rsx!{
				h1 { "Config Error" }
				p { {msg} }
			},
			ConfigContext::Config(_) => rsx! {Router::<Route> {}},
		}
	}
}

fn load_or_create_config_file() -> ConfigContext
{
	match fs::exists(CONFIG_PATH)
	{
		Ok(exists) =>
		{
			if !exists && let Err(err) = fs::write(CONFIG_PATH, "media_dirs = []")
			{
				ConfigContext::Error(format!("Failed to create config file: {}", err))
			}
			else
			{
				ConfigContext::from_file_path(CONFIG_PATH)
			}
		}
		Err(err) => ConfigContext::Error(format!("Failed to read config path: {}", err)),
	}
}
