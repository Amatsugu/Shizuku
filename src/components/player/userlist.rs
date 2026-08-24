use dioxus::prelude::*;

#[component]
pub fn UserList() -> Element
{
	rsx! {
		div{
			id: "users"
		}
	}
}
