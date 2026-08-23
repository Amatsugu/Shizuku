use futures_util::stream::StreamExt;
use std::{
	os::unix::net::UnixStream,
	path::PathBuf,
	process::{Child, Command},
};

use dioxus::{
	hooks::use_coroutine,
	signals::{Memo, ReadableExt},
};

use crate::models::player_context::{PlayerCommands, PlayerContext};

#[cfg(not(windows))]
const IPC_PATH: &str = "/tmp/shizukumpv";
#[cfg(windows)]
const IPC_PATH: &str = r#"\\.\pipe\shizukumpv"#;

#[derive(Debug)]
struct ChildGuard(pub Child);

impl Drop for ChildGuard {
	fn drop(&mut self) {
		println!("drop");
		match self.0.try_wait() {
			Ok(Some(_)) => {}
			Ok(None) => {
				let _ = self.0.kill();
				let _ = self.0.wait();
			}
			Err(_) => {
				let _ = self.0.kill();
			}
		}
	}
}

pub fn start_mpv(mpv_path: Memo<Result<PathBuf, String>>) -> PlayerContext {
	let handle = use_coroutine(
		move |mut rx: dioxus::prelude::UnboundedReceiver<PlayerCommands>| {
			let mut mpv = mpv_path
				.cloned()
				.ok()
				.and_then(spawn_mpv)
				.map(|c| ChildGuard(c));
			async move {
				let Some(mut mpv) = mpv else {
					println!("No Mpv");
					return;
				};
				#[cfg(not(windows))]
				{
					use std::{io::Write, time::Duration};
					tokio::time::sleep(Duration::from_secs(1)).await;
					match tokio::net::UnixStream::connect(IPC_PATH).await {
						Ok(mut stream) => {
							use tokio::io::AsyncWriteExt;

							let (read, mut write) = stream.into_split();
							let mut payload =
								r#"{ "command": ["loadfile", "/home/amatsugu/NFS/Media/VLive/robo4th.mp4"] }"#
									.to_string();
							payload.push('\n');

							if write.write_all(payload.as_bytes()).await.is_err() {
								println!("Failed to write");
							}
							if write.flush().await.is_err() {
								println!("Failed to flush");
							};
						}
						Err(err) => println!("Failed to connect {}", err.to_string()),
					};
				}

				loop {
					tokio::select! {
						Some(cmd) = rx.next() =>{
							match cmd {
								PlayerCommands::OpenFile(_) => todo!(),
								PlayerCommands::Play => todo!(),
								PlayerCommands::Pause => todo!(),
								PlayerCommands::Seek(_) => todo!(),
							}
						}
					}
				}
			}
		},
	);
	PlayerContext { handle }
}

fn spawn_mpv(path: PathBuf) -> Option<Child> {
	Command::new(path)
		.arg("--idle")
		.arg(format!("--input-ipc-server={}", IPC_PATH))
		.spawn()
		.ok()
}
