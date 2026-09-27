use std::borrow::Cow;

use module_common::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShowResultCommandText {
	pub text: String,
}
impl Command for ShowResultCommandText {
	fn module_name() -> Cow<'static, str> {
		"show result".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::Value::String(String::from(&self.text))
	}
}
