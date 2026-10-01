pub mod format_datetime {
	use chrono::{DateTime, Local};

	pub fn long(datetime: &DateTime<Local>) -> String {
		let date_formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
		date_formatted
	}
}

pub mod format_timedelta {
	use chrono::TimeDelta;

	pub fn hms(timedelta: TimeDelta) -> String {
		let secs = timedelta.num_seconds();
		let h = secs / 3600;
		let m = (secs % 3600) / 60;
		let s = secs % 60;

		format!("{h}h {m}m {s}s")
	}
}
