use dioxus::prelude::*;

use crate::{
	components::icons::{Cog, Stack},
	route::Route,
};

#[component]
pub fn NavBar() -> Element {
	rsx! {
		nav {

			Link {
				to: Route::Home {  },
				Stack {}
				"Home"
			}
			Link {
				to: Route::Config {  },
				Cog {}
				"Config"
			}
		}
	}
}
