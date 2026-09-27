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
use modules::common::ResponseExt;

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

fn main() {
	host_app::<AppRoot>();
}
fn host_app<A: App>() {
	let mut args = std::env::args();
	let _arg0 = args.next();

	let mut resp = Response::new();
	let invoke_result = AppRoot::invoke(args, &mut resp);
	match invoke_result {
		Ok(_) => {}
		Err(AppInvokeError::WrongArg {
			arg: arg_received,
			this,
		}) => {
			// show a screen telling u somethings fucked up
			resp.push(modules::ShowResultCommandText {
				text: format!(
					"received incorrect arg {arg_received}\n\n{}",
					this.pretty_print().expect("evil")
				),
			});

			// prompt the user to select an arg that actually exists
			let args_that_were_good = {
				let mut args = std::env::args();
				args.next();
				let mut buf = Vec::new();
				for arg in args {
					if arg != arg_received {
						buf.push(arg);
					} else {
						break;
					}
				}
				buf.join(" ")
			};
			resp.push(prompt_for_menu(&args_that_were_good, &this));
		}
		Err(AppInvokeError::NeedMoreArgs { this }) => {
			let mut args_so_far = std::env::args();
			args_so_far.next();
			let args_so_far = args_so_far.collect::<Vec<_>>().join(" ");

			// show menu containing the args that could be next
			resp.push(prompt_for_menu(&args_so_far, &this))
		}
	}

	let resp_serialized = resp.serialize();
	println!("{resp_serialized}")
}

fn prompt_for_menu(
	args_prefix: &str,
	this: &SubappTree,
) -> modules::MenuAndExecuteCommand<'static> {
	let possible_args = this
		.children
		.iter()
		.map(|a| (a.name, a.desc.unwrap_or(a.name)))
		.map(|(name, desc)| (format!("{args_prefix} {name}"), desc))
		.map(|(name, desc)| (name.into(), desc.into()));

	let desc = match this.desc {
		Some(desc) => format!("\n{desc}"),
		None => String::new(),
	};

	// show menu containing the args that could be next
	modules::MenuAndExecuteCommand {
		prompt: Some(format!("options for {}{}", this.name, desc).into()),
		options: possible_args.collect(),
	}
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
