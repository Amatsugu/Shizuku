use std::path::PathBuf;

use jwalk::WalkDir;

const MEDIA_TYPES: &[&str] = &["mp4", "mkv", "mov", "webm", "avi"];

pub async fn scan_dirs(media_dirs: Vec<String>) -> Vec<PathBuf>
{
	tokio::task::spawn_blocking(move || {
		media_dirs
			.iter()
			.flat_map(|media_dir| {
				WalkDir::new(media_dir).into_iter().filter_map(|e| {
					if let Ok(entry) = e
						&& entry.file_type.is_file()
						&& let Some(ext) = entry.path().extension()
						&& let Some(ext) = ext.to_str()
						&& MEDIA_TYPES.contains(&ext)
					{
						Some(entry.path())
					}
					else
					{
						None
					}
				})
			})
			.collect()
	})
	.await
	.unwrap_or_default()
}
