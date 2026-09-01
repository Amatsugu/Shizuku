use dioxus::prelude::*;

use crate::models::{playlist::PlaylistItem, user::User};

#[derive(Clone, PartialEq, Copy)]
pub struct PlayerContext
{
	pub handle: Coroutine<PlayerCommand>,
	pub data: PlayerData,
}

#[derive(Clone, PartialEq, Copy)]
pub struct PlayerData
{
	pub is_running: Signal<bool>,
	pub playlist: Signal<Vec<PlaylistItem>>,
	pub selected_file: Signal<Option<usize>>,
	pub users: Signal<Vec<User>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerCommand
{
	SelectFile(String),
	AddFile(String, usize),
	Play,
	Pause,
	Seek(u64),
}
