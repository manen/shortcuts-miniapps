use app_common::{App, SubappTree, resp::Response};
use modules::common::ResponseExt;

pub struct TestApp;
impl App for TestApp {
	fn subapp_tree() -> app_common::SubappTree {
		SubappTree {
			name: "test",
			desc: Some("only used for debugging".into()),
			children: vec![],
		}
	}

	fn invoke(
		_args: impl Iterator<Item = String>,
		resp: &mut Response,
	) -> app_common::AppInvokeResult {
		test_new_event(resp);

		Ok(())
	}
}

fn test_new_event(resp: &mut Response) {
	resp.push(modules::NewEventCommand {
		start_time: "2026 okt 1".into(),
		end_time: "2026 okt 1".into(),
		location: "house".into(),
		all_day: true,
		notes: "these are notes".into(),
		title: "jump out the house".into(),
		show_compose_sheet: true,
	});
}
