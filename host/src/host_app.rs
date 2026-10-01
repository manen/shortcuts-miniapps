use app_common::{App, AppInvokeError, SubappReach, SubappTree};
use common::resp::Response;
use modules::{common::ResponseExt, extras::MenuAndExecuteOption};

use crate::AppRoot;

pub fn host_app<A: App>() {
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

		Err(AppInvokeError::Anyhow(err)) => {
			// tell user we fucked up
			resp.push(modules::ShowResultCommandText {
				text: format!("this app encountered an error\n{err}\n\n{err:#?}\n\nsorry"),
			})
		}
	}

	let resp_serialized = resp.serialize();
	println!("{resp_serialized}")
}

fn prompt_for_menu(
	args_prefix: &str,
	this: &SubappTree,
) -> modules::MenuAndExecuteCommand<'static> {
	let args_prefix = if args_prefix.len() == 0 {
		String::new()
	} else {
		format!("{args_prefix} ")
	};

	// let possible_args = this
	// 	.children
	// 	.iter()
	// 	.map(|a| (a.name, a.desc.unwrap_or(a.name)))
	// 	.map(|(name, desc)| (format!("{args_prefix}{name}"), desc))
	// 	.map(|(name, desc)| (name.into(), desc.into()));

	let mut has_fallback = false;
	let possible_args = this
		.children
		.iter()
		.filter_map(|(reach, child)| match reach {
			SubappReach::Literal(new_arg) => {
				// create menu option for this child
				Some(MenuAndExecuteOption {
					name: child.name.into(),
					desc: child.desc.unwrap_or_default().into(),
					execute: modules::ExecuteCommand {
						args: format!("{args_prefix}{new_arg}").into(),
						stdin: "".into(),
					},
				})
			}
			SubappReach::Fallback => {
				has_fallback = true;
				None
			}
		});
	let possible_args = possible_args.collect();

	let desc = match this.desc {
		Some(desc) => format!("\n{desc}"),
		None => String::new(),
	};
	let fallback_note = if has_fallback {
		"\naccepts other arguments not listed here"
	} else {
		""
	};

	// show menu containing the args that could be next
	modules::MenuAndExecuteCommand {
		prompt: Some(format!("{}{}{fallback_note}", this.name, desc).into()),
		options: possible_args,
	}
}
