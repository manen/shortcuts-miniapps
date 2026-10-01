pub use common::resp;
use common::resp::Response;
use std::fmt::Write;

mod branching;

pub trait App {
	fn subapp_tree() -> SubappTree;

	/// if this returns an error, response will be appended with appropriate error actions
	fn invoke(args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult;
}

#[derive(Debug, thiserror::Error)]
pub enum AppInvokeError {
	#[error("need more args")]
	NeedMoreArgs { this: SubappTree },
	#[error("wrong arg supplied")]
	WrongArg { arg: String, this: SubappTree },

	#[error("{0}")]
	Anyhow(#[from] anyhow::Error),
}
pub type AppInvokeResult = std::result::Result<(), AppInvokeError>;

#[derive(Clone, Debug)]
pub enum SubappReach {
	Literal(&'static str),
	Fallback,
}

#[derive(Clone, Debug)]
pub struct SubappTree {
	/// no spaces!
	pub name: &'static str,
	pub desc: Option<&'static str>,
	pub children: Vec<(SubappReach, SubappTree)>,
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

		for (reach, child) in self.children.iter() {
			let child_desc = match child.desc {
				Some(desc) => format!(": {desc}"),
				None => String::new(),
			};

			match reach {
				SubappReach::Literal(child_name) => {
					writeln!(&mut buf, "{} {child_name}{child_desc}", self.name)?;
				}
				SubappReach::Fallback => {
					writeln!(&mut buf, "{} <arg handled by app>{child_desc}", self.name)?;
				}
			}
		}

		Ok(buf)
	}
}
