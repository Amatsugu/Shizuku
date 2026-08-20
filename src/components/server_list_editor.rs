use dioxus::prelude::*;

use crate::{
	components::basic::{Button, ButtonVariant, Input, InputValue},
	models::config_file::{ConfigContext, Server},
};

#[component]
pub fn ServersEditor() -> Element {
	let ctx = use_context::<ConfigContext>();
	let mut cfg = ctx.config;
	let servers = use_memo(move || cfg.cloned().servers);
	let servers = servers();
	rsx! {
		h2 { "Servers" }
		div {
			class: "serversEditor",
			{servers.iter().enumerate().map(|(index, server)|rsx! {
				ServerEditor {
					index,
					server: server.clone(),
					on_remove: move |idx|{
						let mut c = cfg.cloned();
						c.servers.remove(idx);
						cfg.set(c);
					},
					on_update: |(idx, server)|{

					}
				}
			})}
		}
		Button{
			variant: ButtonVariant::Muted,
			onclick: move |_|{
				let mut c = cfg.cloned();
				c.servers.push(Server::default());
				cfg.set(c);
			},
			"Add Server"
		}
	}
}

#[component]
fn ServerEditor(
	server: Server,
	index: usize,
	on_remove: EventHandler<usize>,
	on_update: EventHandler<(usize, Server)>,
) -> Element {
	rsx! {
		div {
			class:"serverEditor",
			div{
				class: "titleBar",
				span { "Server: {server.name}" }
				Button { onclick: move |_| on_remove.call(index), "x" }
			}
			Input{
				value: InputValue::Const(server.name),
				name: "name",
				label: "Name",
				required: true,
			}
			Input{
				value: InputValue::Const(server.host),
				name: "host",
				label: "Host",
				required: true
			}
			Input{
				value: InputValue::Const(server.password.unwrap_or_default()),
				name: "password",
				label: "Passowrd",
				type: "password"
			}
		}
	}
}
