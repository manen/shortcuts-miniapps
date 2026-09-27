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

use common::resp::Response;
use module_common::ResponseExt;

fn main() {
	let mut resp = Response::new();
	resp.push(module_calendar::new_event::NewEventCommand {
		title: "hello".into(),
		start_time: "2026 september 27 12:00 PM".into(),
		end_time: "2026 september 28 12:00 PM".into(),

		..Default::default()
	});
	let commands = resp.serialize();
	println!("{commands}")
}
