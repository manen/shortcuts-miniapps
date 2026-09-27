use std::{borrow::Cow, collections::HashMap};

use module_common::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuAndExecuteCommand<'a> {
	pub prompt: Option<Cow<'a, str>>,
	pub options: HashMap<Cow<'a, str>, Cow<'a, str>>,
}
impl<'a> Command for MenuAndExecuteCommand<'a> {
	fn module_name() -> Cow<'static, str> {
		"menu and execute".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::to_value(self).expect("expect is evil and i should get rid of this")
	}
}
