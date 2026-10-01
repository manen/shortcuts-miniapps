use app_common::{App, AppInvokeError, SubappTree};
use app_gym::GymApp;
use app_test::TestApp;
use common::resp::Response;

mod host_app;
use host_app::host_app;

fn main() {
	host_app::<AppRoot>();
}

const FAKE_SUBAPP_TREE: SubappTree = SubappTree {
	name: "fake",
	desc: Some("this will throw an error"),
	children: vec![],
};

pub struct AppRoot;
impl App for AppRoot {
	fn subapp_tree() -> app_common::SubappTree {
		SubappTree {
			name: "root",
			desc: Some("this is what you get when you call with no args. everything starts here"),
			children: vec![
				GymApp::subapp_tree(),
				FAKE_SUBAPP_TREE,
				TestApp::subapp_tree(),
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
			"gym" => GymApp::invoke(args, resp),
			"test" => TestApp::invoke(args, resp),
			arg => Err(AppInvokeError::WrongArg {
				this: Self::subapp_tree(),
				arg: arg.into(),
			}),
		}
	}
}
