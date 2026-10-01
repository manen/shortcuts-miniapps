use anyhow::Context;
use app_common::{App, AppInvokeResult, SubappTree};
use common::resp::Response;
use modules::common::ResponseExt;
use serde::{Deserialize, Serialize};
use util_db::Db;

// declare GymApp
app_common::branching_app!(
	GymApp, Some("gym app"),

	"start" => GymStart,
	"end" => GymEnd,
	"status" => GymStatus
);

// ---

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum State {
	/// there's no workout going on
	#[default]
	Dormant,
	Started(chrono::DateTime<chrono::Local>),
}

fn db() -> anyhow::Result<Db<State>> {
	let db = util_db::db("gym")
		.with_context(|| format!("while opening gym db to fulfill a gym request"))?;
	Ok(db)
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

	fn invoke(_args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult {
		let mut db = db()?;

		match db.as_ref() {
			State::Started(date) => {
				let date_formatted = util_time::format_datetime::long(date);

				let elapsed = chrono::Local::now() - date;
				let elapsed_formatted = util_time::format_timedelta::hms(elapsed);

				// let the user know we're not overriding the workout that's already happening
				resp.push(modules::ShowNotificationCommand {
					text: format!("there's already a workout going on!\n⏰ {elapsed_formatted}\nstarted {date_formatted}\n\nto start a new workout, end the one that's already started").into(),
					title:Some("gym".into()),
					..Default::default()
				});
				return Ok(());
			}
			_ => {}
		}

		let now = chrono::Local::now();
		let now_formatted = util_time::format_datetime::long(&now);
		db.mutate(|data| *data = State::Started(now))
			.with_context(|| format!("while writing workout start time to db"))?;

		// let the user know it started
		resp.push(modules::ShowNotificationCommand {
			text: format!("workout started at {now_formatted}").into(),
			title: Some("gym".into()),
			..Default::default()
		});

		Ok(())
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

	fn invoke(_args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult {
		let mut db = db()?;

		let old_state = db
			.mutate(|data| {
				let mut swap = State::Dormant;
				std::mem::swap(data, &mut swap);
				swap
			})
			.with_context(|| "while accessing and writing workout end to db")?;

		match old_state {
			State::Started(start_time) => {
				let end_time = chrono::Local::now();

				let start_time_formatted = util_time::format_datetime::long(&start_time);
				let end_time_formatted = util_time::format_datetime::long(&end_time);

				let elapsed = end_time - start_time;
				let elapsed_formatted = util_time::format_timedelta::hms(elapsed);

				// show notif workout ended
				resp.push(modules::ShowNotificationCommand {
					text: format!("🎉 {elapsed_formatted}").into(),
					title: Some("gym".into()),
					..Default::default()
				});
				// add workout to calendar
				resp.push(modules::NewEventCommand {
					title: "gym".into(),
					start_time: start_time_formatted.into(),
					end_time: end_time_formatted.into(),
					..Default::default()
				});
			}
			State::Dormant => {
				// tell user we're not doing shit
				resp.push(modules::ShowNotificationCommand {
					text: "no workout was started anyway".into(),
					..Default::default()
				})
			}
		}

		Ok(())
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

	fn invoke(_args: impl Iterator<Item = String>, resp: &mut Response) -> AppInvokeResult {
		let db = db()?;

		let text = match db.as_ref() {
			State::Started(start_time) => {
				let start_time_formatted = util_time::format_datetime::long(&start_time);

				let elapsed = chrono::Local::now() - start_time;
				let elapsed_formatted = util_time::format_timedelta::hms(elapsed);

				format!("started at {start_time_formatted}\n\n⏰ {elapsed_formatted}")
			}
			State::Dormant => format!("not started"),
		};
		resp.push(modules::ShowResultCommandText { text });

		Ok(())
	}
}
