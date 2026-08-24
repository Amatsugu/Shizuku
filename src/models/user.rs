#[derive(Debug, Clone, PartialEq)]
pub struct User
{
	pub name: String,
	pub is_ready: bool,
	pub selected_file: Option<String>,
}
