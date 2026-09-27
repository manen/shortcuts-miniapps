// async fn start_workout() {
// 	let now = chrono::now();
// 	write_now_to_file().await;

// 	let response = Response::new();
// 	response.show_result("workout started");

// 	response
// }

// async fn end_workout() {
// 	let workout_start = read_workout_start().await;

// 	let response = Response::new();
// 	response.write_to_calendar(CalendarEvent {
// 		name: "gym".into(),
// 		start_time: workout_start,
// 		end_time: chrono::now(),
// 		..Default::default()
// 	});
// 	response.show_result("workout ended: (time elapsed)");

// 	response
// }

use app_common::{App, AppInvokeError, SubappTree};
use app_gym::GymApp;
use common::resp::Response;

// fn main() {
// 	let mut resp = Response::new();
// 	// resp.push(module_calendar::new_event::NewEventCommand {
// 	// 	title: "hello".into(),
// 	// 	start_time: "2026 september 27 12:00 PM".into(),
// 	// 	end_time: "2026 september 28 12:00 PM".into(),

// 	// 	..Default::default()
// 	// });
// 	resp.push(module_execute::menu_and_execute::MenuAndExecuteCommand {
// 		prompt: Some(
// 			format!(
// 				"hello this prompt just came over the wire. args: {}",
// 				std::env::args().collect::<Vec<_>>().join(" ")
// 			)
// 			.into(),
// 		),
// 		options: [("opt1", "and this is where args go? idk"), ("opt2", "blah")]
// 			.into_iter()
// 			.map(|(a, b)| (a.into(), b.into()))
// 			.collect(),
// 	});
// 	let commands = resp.serialize();
// 	println!("{commands}")
// }
//

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
			children: vec![GymApp::subapp_tree(), FAKE_SUBAPP_TREE],
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
			arg => Err(AppInvokeError::WrongArg {
				this: Self::subapp_tree(),
				arg: arg.into(),
			}),
		}
	}
}
