use crate::*;

/// either opens or creates new \
/// don't let users tell you the name cause i'm not filtering for path traversal that much only a little
pub fn db<D: Data>(name: &str) -> Result<Db<D>> {
	let path = format!("./db/{}.json", name.replace("..", ".")).replace("//", "/");
	let path = PathBuf::from(path);
	let mut parent_dir = path.clone();
	parent_dir.pop();

	std::fs::create_dir_all(parent_dir).map_err(Error::CreateDbDir)?;
	let mut file = std::fs::OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(path)
		.map_err(Error::CouldntOpenDb)?;

	match file.try_lock() {
		Ok(a) => a,
		Err(TryLockError::Error(err)) => return Err(Error::FailedToLockIO(err)),
		Err(TryLockError::WouldBlock) => return Err(Error::FailedToLockInUse),
	};

	if file.metadata().map_err(Error::Metadata)?.len() != 0 {
		// db already exists let's read it
		let mut read = std::io::BufReader::new(&mut file);
		let data = serde_json::from_reader(&mut read).map_err(Error::DeserializeRead)?;

		let db = Db {
			file,
			current: data,
		};
		Ok(db)
	} else {
		// db does not exist let's do something
		let mut db = Db {
			file,
			current: Default::default(),
		};

		// fake mutate triggers write
		db.mutate(|_| ())?;
		Ok(db)
	}
}
