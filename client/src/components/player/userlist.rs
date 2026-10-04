use dioxus::prelude::*;

use crate::models::{config_file::ConfigContext, player_context::PlayerContext, user::User};

#[component]
pub fn UserList() -> Element {
	let player_context = use_context::<PlayerContext>();
	let users = player_context.data.users.cloned();
	rsx! {
		div{
			id: "users",
			for user in users {
				UserDisplay { user }
			}
		}
	}
}

#[component]
fn UserDisplay(user: User) -> Element {
	let ready_class = if user.is_ready { "ready" } else { "" };
	let my_username = use_context::<ConfigContext>().config.cloned().username;
	let me = if let Some(my_username) = my_username
		&& user.name == my_username
	{
		"me"
	} else {
		""
	};
	rsx! {
		div{
			class: "user {ready_class} {me}",
			div {
				class: "name",
				{user.name}
			}
		}
	}
}
