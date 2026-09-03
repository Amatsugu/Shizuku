use dioxus::prelude::*;

use crate::{
	components::basic::{Button, ButtonVariant, Input, InputValue, Panel},
	models::config_file::ConfigContext,
	route::Route,
};

#[component]
pub fn Home() -> Element {
	let mut config_ctx = use_context::<ConfigContext>();
	let username = use_memo(move || {
		config_ctx
			.config
			.cloned()
			.username
			.map(|v| InputValue::Const(v))
	});
	rsx! {
		Panel {
			Input {
				type: "text",
				name: "username",
				label: "Username",
				value: username.cloned(),
				oninput: move |e: Event<FormData>|{
					config_ctx.config.write().username = Some(e.value());
				}
			}
			Button{
				variant: ButtonVariant::Accented,
				onclick: |_|{
					navigator().push(Route::Player {  });
				},
				"Start"
			}
		}
	}
}
