use std::borrow::Cow;

use anyhow::Context;
use app_common::{App, AppInvokeResult, resp::Response};
use modules::{ExecuteCommand, common::ResponseExt, extras::MenuAndExecuteOption};
use serde::{Deserialize, Serialize};
use util_db::Db;

app_common::branching_app!(
	DateTrackerApp, Some("track dates and times of whatever"),

	"track_event" => TrackEventWhatType,
	"track_event_confirm" => TrackEventConfirm
);

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
/// this is what's saved in the database
pub struct EventTypeEntry {
	/// if set, also appends the new line to a note on the iphone \
	/// this is to keep a backup or something
	corresponding_note_title: Option<String>,
	list: Vec<String>,
}

/// untrusted str to db
fn db(event_type: &str) -> anyhow::Result<Db<EventTypeEntry>> {
	let event_type = event_type.trim_matches('/');
	let db_name = format!("date_tracker/{event_type}");
	let db: Db<EventTypeEntry> =
		util_db::db(&db_name).with_context(|| format!("failed to open db {db_name}"))?;
	Ok(db)
}

app_common::invoking_app!(
	TrackEventWhatType = "track_event",
	Some("track event"),
	track_event_what_type
);
fn track_event_what_type(
	mut args: impl Iterator<Item = String>,
	resp: &mut Response,
) -> AppInvokeResult {
	match args.next() {
		Some(event_type) => {
			let db = db(&event_type)?;

			{
				let db = db.as_ref();
				let count = db.list.len();

				let mut last_3_lines = db.list.iter().rev().take(5).cloned().collect::<Vec<_>>();
				last_3_lines.reverse();
				let last_3_lines = last_3_lines.join("\n");

				let now = chrono::Local::now();
				let now_formatted = util_time::format_datetime::long(&now);

				let display = format!("{event_type}: {count}\n\n{last_3_lines}\n\n{now_formatted}");
				resp.push(modules::ShowResultCommandText { text: display });
				resp.push(modules::ExecuteCommand {
					args: format!("date_tracker track_event_confirm {event_type} {now_formatted}")
						.into(),
					stdin: Default::default(),
				});
			}
		}
		None => {
			// if there's no args
			let date_types = util_db::dbs_list("date_tracker")
				.with_context(|| format!("when querying what types of things can be tracked"))?;

			resp.push(modules::MenuAndExecuteCommand {
				prompt: Some("what type of thing just happened".into()),
				options: date_types
					.map(|db_name| {
						let name_clean = db_name
							.split('/')
							.last()
							.map(|last| last)
							.unwrap_or_else(|| &db_name)
							.to_string();

						// println!("{name_clean}");

						let opt = MenuAndExecuteOption {
							name: name_clean.clone().into(),
							desc: name_clean.clone().into(),
							execute: ExecuteCommand {
								args: format!("date_tracker track_event {name_clean}").into(),
								stdin: "".into(),
							},
						};
						// println!("{opt:#?}");

						opt
					})
					.collect(),
			});
		}
	};

	Ok(())
}

// confirm:

app_common::invoking_app!(
	TrackEventConfirm = "track_event_confirm",
	Some("track event confirm don't use this one actually"),
	track_event_confirm
);
fn track_event_confirm(
	mut args: impl Iterator<Item = String>,
	resp: &mut Response,
) -> AppInvokeResult {
	// extract info from args
	let type_name = args
		.next()
		.ok_or_else(|| anyhow::anyhow!("no db name provided"))?;

	let to_append = args.collect::<Vec<String>>().join(" ");

	// write to db
	let mut db = db(&type_name)?;
	db.mutate(|db| {
		db.list.push(to_append.clone());
	})
	.with_context(|| format!("while mutating db to append {to_append} to {type_name}"))?;

	// write to corresponding note if exists
	match &db.as_ref().corresponding_note_title {
		Some(title) => {
			resp.push(modules::AppendToNoteCommand {
				name_contains: Cow::Borrowed(&title),
				append: Cow::Borrowed(&to_append),
				..Default::default()
			});
		}
		None => {}
	}

	// whatever

	Ok(())
}
