use dioxus::prelude::*;

use crate::{
	components::basic::{Button, ButtonVariant},
	route::Route,
};

#[component]
pub fn Home() -> Element
{
	rsx! {
		Link{
			to: Route::Player {  },
			Button{
				variant: ButtonVariant::Accented,
				"Start"
			}
		}
	}
}
