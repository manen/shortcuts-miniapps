use std::borrow::Cow;

use common::resp::{Action, Response};

pub trait Command {
	fn module_name() -> Cow<'static, str>;
	fn data(&self) -> serde_json::Value;
}

pub trait ResponseExt {
	fn push<C: Command>(&mut self, command: C);
}
impl ResponseExt for Response {
	fn push<C: Command>(&mut self, command: C) {
		let action = Action::new(C::module_name(), command.data());
		self.actions.push(action);
	}
}
