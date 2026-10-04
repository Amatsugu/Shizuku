use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct MpvEvent
{
	pub event: String,
	#[serde(flatten)]
	extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct MpvReply
{
	pub error: String,
	#[serde(default)]
	data: Option<Value>,
	#[serde(default)]
	request_id: Option<u64>,
}

pub enum MpvMessage
{
	Event(MpvEvent),
	Reply(MpvReply),
}

impl MpvMessage
{
	pub fn parse(data: String) -> Result<Self, serde_json::Error>
	{
		let value: Value = serde_json::from_str(data.trim())?;
		if value.get("event").is_some()
		{
			Ok(MpvMessage::Event(serde_json::from_value(value)?))
		}
		else
		{
			Ok(MpvMessage::Reply(serde_json::from_value(value)?))
		}
	}
}
