use std::time::Duration;

use dioxus::prelude::*;

use crate::{
	components::basic::{Button, ButtonVariant, Input, InputValue, Panel},
	models::{
		config_file::ConfigContext,
		toasts::{ToastCommand, ToastsContext},
	},
	route::Route,
};

#[component]
pub fn Home() -> Element {
	let mut config_ctx = use_context::<ConfigContext>();
	let toasts_ctx = use_context::<ToastsContext>();
	let username = use_memo(move || {
		config_ctx
			.config
			.cloned()
			.username
			.map(|v| InputValue::Const(v))
	});
	let start = move |_| {
		if config_ctx.config.cloned().username.is_some() {
			navigator().push(Route::Player {});
		} else {
			toasts_ctx.handle.send(
				ToastCommand::push_error_with_message(
					"Username is required",
					"Username cannot be empty",
				)
				.with_duration(Duration::from_secs(5)),
			);
		}
	};
	rsx! {
		Panel {
			Input {
				type: "text",
				name: "username",
				label: "Username",
				value: username.cloned(),
				required: true,
				oninput: move |e: Event<FormData>|{
					if e.valid(){
						config_ctx.config.write().username = Some(e.value());
					}
				}
			}
			Button{
				variant: ButtonVariant::Accented,
				onclick: start,
				"Start"
			}
		}
	}
}
