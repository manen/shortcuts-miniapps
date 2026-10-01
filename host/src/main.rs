use app_common::{App, AppInvokeError, SubappTree};
use app_gym::GymApp;
use app_test::TestApp;
use common::resp::Response;

mod host_app;
use host_app::host_app;

app_common::branching_app!(
	AppRoot, None,

	"gym" => GymApp,
	"test" => TestApp,
);

fn main() {
	host_app::<AppRoot>();
}
