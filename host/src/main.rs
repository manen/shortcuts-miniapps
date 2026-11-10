use app_common::App;
use app_date_tracker::DateTrackerApp;
use app_gym::GymApp;
use app_test::TestApp;
use common::resp::Response;

mod host_app;
use host_app::host_app;

app_common::branching_app!(
	AppRoot, None,

	"gym" => GymApp,
	"test" => TestApp,
	"date_tracker" => DateTrackerApp,
);

fn main() {
	host_app::<AppRoot>();
}
