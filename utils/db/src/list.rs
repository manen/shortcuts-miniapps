use crate::*;

/// input the parent directory to list dbs in, returns full paths of dbs that exist in that dir
pub fn dbs_list(dir: &str) -> Result<impl Iterator<Item = String>> {
	let path = format!("./db/{}", dir.replace("..", ".")).replace("//", "/");
	let path = path.trim_end_matches('/').to_owned();
	let path = PathBuf::from(&path);

	let readdir = std::fs::read_dir(&path);
	let readdir = match readdir {
		Ok(a) => a,
		Err(err) => {
			return match err.kind() {
				std::io::ErrorKind::NotFound => Err(Error::FolderDoesntExit {
					dir_supplied: dir.to_owned(),
				}),
				_ => Err(Error::Metadata(err)),
			};
		}
	};

	let collected = readdir
		.into_iter()
		.collect::<Result<Vec<_>, _>>()
		.map_err(Error::ReadDirWhileReading)?;

	let iter = collected.into_iter();

	let checked_for_file = iter.map(|entry| entry.metadata().map(|a| (entry, a.is_file())));
	let collected = checked_for_file
		.collect::<Result<Vec<_>, _>>()
		.map_err(Error::Metadata)?;
	let checked_for_file = collected
		.into_iter()
		.filter_map(|(entry, keep)| if keep { Some(entry) } else { None });

	let names_only =
		checked_for_file.map(|entry| String::from(entry.file_name().to_string_lossy()));
	let checked_for_json = names_only.filter(|name| name.ends_with(".json"));

	let absolute = checked_for_json
		.map(move |name| format!("{dir}/{}", name.strip_suffix(".json").unwrap_or(&name)));

	Ok(absolute)
}
