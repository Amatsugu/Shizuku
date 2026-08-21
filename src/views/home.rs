use std::time::Duration;

use dioxus::prelude::*;

use crate::{
	components::basic::{Button, ButtonVariant},
	models::toasts::{ToastCommand, ToastLevel, ToastsContext},
	route::Route,
};

#[component]
pub fn Home() -> Element
{
	let toasts_ctx = use_context::<ToastsContext>();
	rsx! {
		Link{
			to: Route::Player {  },
			Button{
				variant: ButtonVariant::Accented,
				"Start"
			}
		}
		Button{
			variant: ButtonVariant::Muted,
			onclick: move |_|{
				toasts_ctx.handle.send(ToastCommand::PushWithDuration{
					title: "Test".into(),
					message: Some("This is a test toast".into()),
					level: ToastLevel::Info,
					duration: Duration::from_secs_f32(10.0)
				});
			},
			"Show Toast"
		}
	}
}
