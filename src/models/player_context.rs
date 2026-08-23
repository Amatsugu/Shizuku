use dioxus::prelude::*;

#[derive(Clone, PartialEq, Copy)]
pub struct PlayerContext
{
	pub handle: Coroutine<PlayerCommand>,
	pub is_running: Signal<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerCommand
{
	OpenFile(String),
	Play,
	Pause,
	Seek(u64),
}
