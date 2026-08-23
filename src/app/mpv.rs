use std::{
	path::PathBuf,
	process::{Child, Command},
	thread,
	time::Duration,
};

use interprocess::local_socket::{GenericFilePath, RecvHalf, SendHalf, Stream, ToFsName, prelude::*};

#[cfg(not(windows))]
const IPC_PATH: &str = "/tmp/shizukumpv";
#[cfg(windows)]
const IPC_PATH: &str = r#"\\.\pipe\shizukumpv"#;

pub struct Mpv
{
	pub process: ChildGuard,
	pub read: RecvHalf,
	pub write: SendHalf,
}

#[derive(Debug)]
struct ChildGuard(pub Child);

impl Drop for ChildGuard
{
	fn drop(&mut self)
	{
		println!("drop");
		match self.0.try_wait()
		{
			Ok(Some(_)) =>
			{}
			Ok(None) =>
			{
				let _ = self.0.kill();
				let _ = self.0.wait();
			}
			Err(_) =>
			{
				let _ = self.0.kill();
			}
		}
	}
}

impl Mpv
{
	pub fn start_mpv(path: PathBuf) -> Result<Self, String>
	{
		let mpv = spawn_mpv(path).map_err(|e| e.to_string())?;
		let ipc_name = IPC_PATH.to_fs_name::<GenericFilePath>().map_err(|e| e.to_string())?;
		const MAX_ATTEMPTS: u64 = 5;
		let mut cur_attempts = 0;
		loop
		{
			match Stream::connect(ipc_name.clone())
			{
				Ok(ipc) =>
				{
					let (read, write) = ipc.split();
					return Ok(Self {
						process: ChildGuard(mpv),
						read,
						write,
					});
				}
				Err(e) if cur_attempts > MAX_ATTEMPTS => return Err(e.to_string()),
				Err(_) =>
				{
					thread::sleep(Duration::from_secs(cur_attempts * 2));
					cur_attempts += 1;
				}
			}
		}
	}
}

fn spawn_mpv(path: PathBuf) -> std::io::Result<Child>
{
	Command::new(path)
		.arg("--idle")
		.arg(format!("--input-ipc-server={}", IPC_PATH))
		.spawn()
}
