use dioxus::prelude::*;

use crate::{
	components::basic::{Button, ButtonVariant},
	route::Route,
};

#[component]
pub fn PlayerControls() -> Element {
	rsx! {
		div{
			id: "playerControls",
			Link{
				to: Route::Home {},
				Button {
					variant: ButtonVariant::Cancel,
					"Exit"
				}
			}
		}
	}
}
