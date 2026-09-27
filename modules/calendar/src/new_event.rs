use std::borrow::Cow;

use module_common::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewEventCommand<'a> {
	pub title: Cow<'a, str>,
	pub start_time: Cow<'a, str>,
	pub end_time: Cow<'a, str>,

	pub location: Cow<'a, str>,
	pub all_day: bool,

	pub show_compose_sheet: bool,
}
impl<'a> Command for NewEventCommand<'a> {
	fn module_name() -> Cow<'static, str> {
		"calendar new event".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::to_value(self).expect("expect is evil but i can't be bothered this is a poc")
	}
}
