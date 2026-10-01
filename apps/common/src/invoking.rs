#[macro_export]
macro_rules! invoking_app {
	(
		$app:ident
		$(= $name:literal)?
		, $desc:expr,
		$invoke:ident
	) => {
		pub struct $app;

		impl App for $app {
			fn subapp_tree() -> app_common::SubappTree {
				app_common::SubappTree {
					name: $crate::invoking_app!(@name $app $(, $name)?),
					desc: $desc,
					children: Default::default(),
				}
			}

			fn invoke(
				args: impl Iterator<Item = String>,
				resp: &mut Response,
			) -> app_common::AppInvokeResult {
				$invoke(args, resp)
			}
		}
	};

	(@name $app:ident, $name:literal) => {
		$name
	};

	(@name $app:ident) => {
		stringify!($app)
	};
}
