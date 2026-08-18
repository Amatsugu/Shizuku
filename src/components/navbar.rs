use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn NavBar() -> Element {
	rsx! {
		Link {
			to: Route::Home {  },
			"Home"
		}
		Link {
			to: Route::Config {  },
			"Config"
		}
	}
}
