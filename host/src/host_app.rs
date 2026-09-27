use app_common::{App, AppInvokeError, SubappTree};
use common::resp::Response;
use modules::common::ResponseExt;

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
