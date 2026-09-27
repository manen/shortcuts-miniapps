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
use module_common::ResponseExt;

fn main() {
	let mut resp = Response::new();
	// resp.push(module_calendar::new_event::NewEventCommand {
	// 	title: "hello".into(),
	// 	start_time: "2026 september 27 12:00 PM".into(),
	// 	end_time: "2026 september 28 12:00 PM".into(),

	// 	..Default::default()
	// });
	resp.push(module_execute::menu_and_execute::MenuAndExecuteCommand {
		prompt: Some(
			format!(
				"hello this prompt just came over the wire. args: {}",
				std::env::args().collect::<Vec<_>>().join(" ")
			)
			.into(),
		),
		options: [("opt1", "and this is where args go? idk"), ("opt2", "blah")]
			.into_iter()
			.map(|(a, b)| (a.into(), b.into()))
			.collect(),
	});
	let commands = resp.serialize();
	println!("{commands}")
}

// fn main() {
// 	host_app::<AppRoot>();
// }
// fn host_app<A: App>() {
// 	let mut args = std::env::args();
// 	let _arg0 = args.next();

// 	let mut resp = Response::new();
// 	let invoke_result = AppRoot::invoke(args, &mut resp);
// 	match invoke_result {
// 		Ok(resp) => {
// 			let resp_serialized = resp.serialize();
// 			println!("{resp_serialized}")
// 		}
// 		Err(typ) => {
// 			eprintln!("app didn't run successfully! {}", typ.pretty_print());
// 			let this = match typ {
// 				AppInvokeError::NeedMoreArgs { this } | AppInvokeError::WrongArg { this } => this,
// 			};

// 			let txt = this.pretty_print().expect("fuck you");
// 			eprintln!("\n{txt}");
// 		}
// 	}
// }

// pub struct AppRoot;
// impl App for AppRoot {
// 	fn subapp_tree() -> app_common::SubappTree {
// 		SubappTree {
// 			name: "root",
// 			desc: Some("this is what you get when you call with no args. everything starts here"),
// 			children: vec![GymApp::subapp_tree()],
// 		}
// 	}
// 	fn invoke(
// 		mut args: impl Iterator<Item = String>,
// 		resp: &mut Response,
// 	) -> app_common::AppInvokeResult {
// 		let next = args.next().ok_or(AppInvokeError::NeedMoreArgs {
// 			this: Self::subapp_tree(),
// 		})?;
// 		match next.as_ref() {
// 			"gym" => GymApp::invoke(args, resp),
// 			_ => Err(AppInvokeError::WrongArg {
// 				this: Self::subapp_tree(),
// 			}),
// 		}
// 	}
// }
