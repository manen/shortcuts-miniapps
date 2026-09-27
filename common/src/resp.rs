use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Response {
	pub actions: Vec<Action>,
}
impl Response {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn serialize(self) -> String {
		serde_json::to_string(&self).expect("expect is evil but i'm not dealing with errors yet")
	}
}

// --

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
	module_name: Cow<'static, str>,
	data: serde_json::Value,
}
impl Action {
	pub fn new(module_name: impl Into<Cow<'static, str>>, data: serde_json::Value) -> Self {
		Self {
			module_name: module_name.into(),
			data,
		}
	}
}
