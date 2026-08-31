use dioxus::prelude::*;

use crate::route::Route;
const PLAYER_CSS: Asset = asset!("/assets/player.scss");

#[component]
pub fn PlayerLayout() -> Element
{
	rsx! {
		document::Link { rel: "stylesheet", href: PLAYER_CSS }
		div {
			id: "playerContent",
			Outlet::<Route>{}
		 }
	}
}
