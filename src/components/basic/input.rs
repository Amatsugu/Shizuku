use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps
{
	pub r#type: Option<String>,
	pub value: Option<InputValue>,
	pub label: Option<String>,
	pub placeholder: Option<String>,
	pub name: String,
	pub oninput: Option<EventHandler<Event<FormData>>>,
	pub required: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputValue
{
	Const(String),
	Signal(Signal<String>),
}

impl From<String> for InputValue
{
	fn from(value: String) -> Self
	{
		InputValue::Const(value)
	}
}

impl From<Signal<String>> for InputValue
{
	fn from(value: Signal<String>) -> Self
	{
		InputValue::Signal(value)
	}
}

impl InputValue
{
	pub fn value(&self) -> String
	{
		match self
		{
			InputValue::Const(val) => val.clone(),
			InputValue::Signal(signal) => signal.cloned(),
		}
	}
}

#[component]
pub fn Input(props: InputProps) -> Element
{
	let label = props.label.unwrap_or_default();
	let mut val = props.value.clone();
	let ph = props.placeholder.unwrap_or(label.clone());
	rsx! {
		label {
			{label}
			input {
				r#type: props.r#type.unwrap_or("text".into()),
				value: props.value.map(|e|e.value()).unwrap_or_default(),
				oninput: move |e| {
					if let Some(val) = &mut val && let InputValue::Signal(sig) = val {
						sig.set(e.value());
					}
					if let Some(handler) = props.oninput{
						handler.call(e);
					}
				},
				name: props.name,
				placeholder: ph,
				required: props.required,
			}
		}
	}
}
