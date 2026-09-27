use app_common::{App, AppInvokeError, AppInvokeResult, SubappTree};
use common::resp::Response;

pub struct GymApp;
impl App for GymApp {
	fn subapp_tree() -> app_common::SubappTree {
		SubappTree {
			name: "gym",
			desc: Some("gym start, end, status"),
			children: vec![
				GymStart::subapp_tree(),
				GymEnd::subapp_tree(),
				GymStatus::subapp_tree(),
			],
		}
	}

	fn invoke(
		mut args: impl Iterator<Item = String>,
		resp: &mut Response,
	) -> app_common::AppInvokeResult {
		let next = args.next().ok_or(AppInvokeError::NeedMoreArgs {
			this: Self::subapp_tree(),
		})?;
		match next.as_ref() {
			"start" => GymStart::invoke(args, resp),
			"end" => GymEnd::invoke(args, resp),
			"status" => GymStatus::invoke(args, resp),
			arg => Err(AppInvokeError::WrongArg {
				this: Self::subapp_tree(),
				arg: arg.into(),
			}),
		}
	}
}

pub struct GymStart;
pub struct GymEnd;
pub struct GymStatus;

impl App for GymStart {
	fn subapp_tree() -> SubappTree {
		SubappTree {
			name: "start",
			desc: Some("starts the gym session but nothing interesting yet just saves the time"),
			children: Default::default(),
		}
	}

	fn invoke(args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult {
		todo!()
	}
}
impl App for GymEnd {
	fn subapp_tree() -> SubappTree {
		SubappTree {
			name: "end",
			desc: Some(
				"ends the gym session, shows an overview and adds a calendar event for the gym session",
			),
			children: Default::default(),
		}
	}

	fn invoke(args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult {
		todo!()
	}
}
impl App for GymStatus {
	fn subapp_tree() -> SubappTree {
		SubappTree {
			name: "status",
			desc: Some("shows the status of the current gym session"),
			children: Default::default(),
		}
	}

	fn invoke(args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult {
		todo!()
	}
}
