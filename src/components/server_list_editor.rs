use dioxus::prelude::*;

use crate::{
	components::{
		basic::{Button, ButtonVariant, Input, InputValue},
		icons::Cross,
	},
	models::config_file::{ConfigContext, Server},
};

#[component]
pub fn ServersEditor() -> Element
{
	let ctx = use_context::<ConfigContext>();
	let servers = use_memo(move || ctx.config.cloned().servers);
	let mut cfg = ctx.config;
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
						cfg.write().servers.remove(idx);
					},
					on_update: move |(idx, server)|{
						let mut c = cfg.cloned();
						if c.servers.len() > idx{
							c.servers[idx] = server;
							cfg.set(c);
						}
					}
				}
			})}
		}
		Button{
			variant: ButtonVariant::Muted,
			onclick: move |_|{
				cfg.write().servers.push(Server::default());
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
) -> Element
{
	rsx! {
		div {
			class:"serverEditor",
			div{
				class: "titleBar",
				span { "Server: {server.name.clone()}" }
				Button { onclick: move |_| on_remove.call(index), Cross {} }
			}
			Input{
				value: InputValue::Const(server.name.clone()),
				name: "name",
				label: "Name",
				required: true,
				oninput: {
					let server = server.clone();
					move |e: Event<FormData>| on_update.call((index, Server{ name: e.value(), ..server.clone() }))
				}
			}
			Input{
				value: InputValue::Const(server.host.clone()),
				name: "host",
				label: "Host",
				required: true,
				oninput: {
					let server = server.clone();
					move |e: Event<FormData>| on_update.call((index, Server{ host: e.value(), ..server.clone() }))
				}
			}
			Input{
				value: InputValue::Const(server.password.clone().unwrap_or_default()),
				name: "password",
				label: "Password",
				type: "password",
				oninput: {
					let server = server.clone();
					move |e: Event<FormData>| on_update.call((index, Server{ password: if e.value().is_empty() { None }else{ Some(e.value()) }, ..server.clone() }))
				}
			}
		}
	}
}
