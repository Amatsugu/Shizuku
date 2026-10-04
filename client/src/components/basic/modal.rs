use dioxus::prelude::*;

#[component]
pub fn Panel(children: Element) -> Element {
	rsx! {
		div{
			class: "panel",
			{children}
		}
	}
}
