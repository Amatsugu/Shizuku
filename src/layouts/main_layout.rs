use dioxus::prelude::*;

use crate::{components::NavBar, route::Route};

#[component]
pub fn MainLayout() -> Element
{
	rsx! {
		NavBar {},
		div {
			id: "content",
			Outlet::<Route>{}
		}
	}
}
