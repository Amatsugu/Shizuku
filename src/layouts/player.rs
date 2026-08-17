use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn PlayerLayout() -> Element
{
	rsx! {
		div {
			id: "playerContent",
			Outlet::<Route>{}
		 }
	}
}
