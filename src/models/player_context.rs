use dioxus::html::HasFileData;
use dioxus::prelude::*;

use crate::{
	app::file_scanner::MEDIA_TYPES,
	models::{playlist::PlaylistItem, user::User},
};

#[derive(Clone, PartialEq, Copy)]
pub struct PlayerContext
{
	pub handle: Coroutine<PlayerCommand>,
	pub data: PlayerData,
}

impl PlayerContext
{
	pub fn handle_reorder(mut self, from: Option<usize>, mut to: usize, items: &Vec<PlaylistItem>)
	{
		if let Some(from) = from
		{
			let mut items = items.clone();
			if from < to
			{
				to = to - 1;
			}
			let item = items.remove(from);
			items.insert(to, item);
			self.data.playlist.set(items);
		}
	}

	pub fn handle_file_drop(self, event: Event<DragData>, index: usize)
	{
		event
			.files()
			.iter()
			.filter(|f| {
				f.path()
					.extension()
					.and_then(|e| e.to_str())
					.map(|e| MEDIA_TYPES.contains(&e))
					.unwrap_or_default()
			})
			.filter_map(|f| f.path().to_str().map(|f| f.to_string()))
			.for_each(|f| {
				self.handle.send(PlayerCommand::AddFile(f, index));
			});
	}
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
