use std::borrow::Cow;

use module_common::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ShowNotificationCommand<'a> {
	pub text: Cow<'a, str>,
	pub title: Option<Cow<'a, str>>,
	pub play_sound: bool,
}
impl<'a> Command for ShowNotificationCommand<'a> {
	fn module_name() -> Cow<'static, str> {
		"show notification".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::to_value(self).expect("evil expect")
	}
}
