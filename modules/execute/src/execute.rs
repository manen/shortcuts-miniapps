use std::borrow::Cow;

use module_common::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteCommand<'a> {
	pub args: Cow<'a, str>,
}
impl<'a> Command for ExecuteCommand<'a> {
	fn module_name() -> Cow<'static, str> {
		"executor".into()
	}
	fn data(&self) -> serde_json::Value {
		serde_json::to_value(&self.args).expect("expect is evil and i should get rid of this")
	}
}
