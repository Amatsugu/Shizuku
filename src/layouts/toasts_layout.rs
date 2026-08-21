use dioxus::prelude::*;

use crate::{
	components::toasts::{ToastsDisplay, init_toasts},
	route::Route,
};

#[component]
pub fn ToastsLayout() -> Element
{
	let ctx = init_toasts();
	use_context_provider(|| ctx);
	rsx! {
		ToastsDisplay {}
		Outlet::<Route> {}
	}
}
