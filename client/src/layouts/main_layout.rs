use dioxus::prelude::*;

use crate::{components::NavBar, route::Route};

const MAIN_CSS: Asset = asset!("/assets/main.scss");
#[component]
pub fn MainLayout() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		NavBar {},
		div {
			id: "content",
			Outlet::<Route>{}
		}
	}
}
