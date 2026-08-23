use dioxus::prelude::*;

#[derive(Clone, PartialEq, Copy)]
pub struct PlayerContext {
	pub handle: Coroutine<PlayerCommands>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerCommands {
	OpenFile(String),
	Play,
	Pause,
	Seek(u32),
}
