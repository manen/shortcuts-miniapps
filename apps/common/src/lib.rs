use std::fmt::Write;

use common::resp::Response;

pub trait App {
	fn subapp_tree() -> SubappTree;

	/// if this returns an error, response will be appended with appropriate error actions
	fn invoke(args: impl Iterator<Item = String>, response: &mut Response) -> AppInvokeResult;
}

#[derive(Clone, Debug)]
pub enum AppInvokeError {
	NeedMoreArgs { this: SubappTree },
	WrongArg { this: SubappTree },
}
impl AppInvokeError {
	pub fn pretty_print(&self) -> String {
		match self {
			Self::NeedMoreArgs { .. } => format!("need more args"),
			Self::WrongArg { .. } => format!("wrong arg"),
		}
	}
}
pub type AppInvokeResult = std::result::Result<Response, AppInvokeError>;

#[derive(Clone, Debug)]
pub struct SubappTree {
	/// no spaces!
	pub name: &'static str,
	pub desc: Option<&'static str>,
	pub children: Vec<SubappTree>,
}
impl SubappTree {
	pub fn pretty_print(&self) -> Result<String, std::fmt::Error> {
		let mut buf = String::new();
		writeln!(&mut buf, "")?;
		writeln!(&mut buf, "{}", self.name)?;

		match &self.desc {
			Some(desc) => {
				writeln!(&mut buf, " -- {desc}")?;
			}
			None => {}
		}

		writeln!(&mut buf, "")?;

		for c in self.children.iter() {
			let desc = match c.desc {
				Some(desc) => format!(": {desc}"),
				None => String::new(),
			};
			writeln!(&mut buf, "{} {}{}", self.name, c.name, desc)?;
		}

		Ok(buf)
	}
}
