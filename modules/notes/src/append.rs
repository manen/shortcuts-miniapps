use std::borrow::Cow;

use module_common::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AppendToNoteCommand<'a> {
	pub name_contains: Cow<'a, str>,
	pub append: Cow<'a, str>,

	pub section_title: Option<Cow<'a, str>>,
	pub ignore_empty_lines: bool,
	pub interpret_as_markdown: bool,
}
impl<'a> Command for AppendToNoteCommand<'a> {
	fn module_name() -> Cow<'static, str> {
		"append to note".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::to_value(self).expect("expect is STILL evil")
	}
}
