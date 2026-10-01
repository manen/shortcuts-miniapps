use std::borrow::Cow;

use module_common::Command;
use serde::{Deserialize, Serialize};

use crate::execute::ExecuteCommand;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuAndExecuteOption<'a> {
	pub name: Cow<'a, str>,
	pub desc: Cow<'a, str>,

	pub execute: ExecuteCommand<'a>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuAndExecuteCommand<'a> {
	pub prompt: Option<Cow<'a, str>>,
	pub options: Vec<MenuAndExecuteOption<'a>>,
}
impl<'a> Command for MenuAndExecuteCommand<'a> {
	fn module_name() -> Cow<'static, str> {
		"menu and execute".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::to_value(self).expect("expect is evil and i should get rid of this")
	}
}
