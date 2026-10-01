use app_common::{App, SubappTree, resp::Response};
use modules::common::ResponseExt;

app_common::branching_app!(
	TestApp, Some("geci"),

	"new_event" => TestNewEvent,
	"nested" => TestNested
);
app_common::branching_app!(TestNested = "nested", Some("deep nesting"),
	"further" => TestNestedFurther
);
app_common::branching_app!(TestNestedFurther = "further", Some("deeper nesting"),
	"yea" => TestNestedFurtherYea
);
app_common::branching_app!(TestNestedFurtherYea = "yea", Some("deeper nesting"),
	"new_event" => TestNewEvent
);

pub struct TestNewEvent;
impl App for TestNewEvent {
	fn subapp_tree() -> app_common::SubappTree {
		SubappTree {
			name: "new_event",
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
