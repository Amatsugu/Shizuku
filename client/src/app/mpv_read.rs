use dioxus::logger::tracing::error;

use crate::models::mpv::responses::MpvMessage;

pub fn handle_mpv_read(read_line: std::io::Result<Option<String>>)
{
	match read_line
	{
		Ok(Some(line)) =>
		{
			let _reply = MpvMessage::parse(line);
		}
		Ok(None) =>
		{}
		Err(err) =>
		{
			error!("Failed to read mpv stream: {}", err);
		}
	}
}
