// imma keep it a buck this whole macro is ai generated but it works great
// important thing to note: child's self-described name needs to match the name the parent branch calls the child

#[macro_export]
macro_rules! branching_app {
	(
		$app:ident
		$(= $name:literal)?
		, $desc:expr,
		$(
			$child_name:literal => $child:ident
		),* $(,)?
	) => {
		pub struct $app;

		impl App for $app {
			fn subapp_tree() -> app_common::SubappTree {
				app_common::SubappTree {
					name: $crate::branching_app!(@name $app $(, $name)?),
					desc: $desc,
					children: vec![
						$(
							(app_common::SubappReach::Literal($child_name), $child::subapp_tree()),
						)*
					],
				}
			}

			fn invoke(
				mut args: impl Iterator<Item = String>,
				resp: &mut Response,
			) -> app_common::AppInvokeResult {
				let next = args.next().ok_or(
					app_common::AppInvokeError::NeedMoreArgs {
						this: Self::subapp_tree(),
					}
				)?;

				match next.as_ref() {
					$(
						$child_name => $child::invoke(args, resp),
					)*
					arg => Err(app_common::AppInvokeError::WrongArg {
						this: Self::subapp_tree(),
						arg: arg.into(),
					}),
				}
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
