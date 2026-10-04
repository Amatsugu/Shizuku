#[derive(Debug, Clone, PartialEq)]
pub struct User {
	pub name: String,
	pub is_ready: bool,
	pub selected_file: Option<String>,
}

impl User {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			is_ready: false,
			selected_file: None,
		}
	}
}
