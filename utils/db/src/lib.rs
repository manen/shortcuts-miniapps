use std::{
	fs::{File, TryLockError},
	io::Seek,
	path::PathBuf,
};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("failed to create parent dir for db:\n{0}")]
	CreateDbDir(std::io::Error),
	#[error("failed to open the file for database:\n{0}")]
	CouldntOpenDb(std::io::Error),
	#[error("failed to lock db because of an io error\n{0}")]
	FailedToLockIO(std::io::Error),
	#[error("failed to lock db because it's in use already")]
	FailedToLockInUse,
	#[error("failed to read metadata for db file\n{0}")]
	Metadata(std::io::Error),

	#[error("failed to deserialize and/or read data inside db\n{0}")]
	DeserializeRead(serde_json::error::Error),
	#[error("failed to serialize and/or write data inside db\n{0}")]
	SerializeWrite(serde_json::error::Error),

	#[error("failed to truncate old db file when writing new contents (set_len failed)\n{0}")]
	TruncateWhenWrite(std::io::Error),
	#[error("failed to seek to beginning of db file when writing changed contents\n{0}")]
	SeekWhenWrite(std::io::Error),
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait Data: Serialize + DeserializeOwned + Default {}
impl<T> Data for T where T: Serialize + DeserializeOwned + Default {}

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

/// essentially a simple database over any serializable type D \
/// keeps a lock over the file it's stored in so other instances of the app don't fuck it up
pub struct Db<D: Data> {
	file: File,
	current: D,
}
impl<D: Data> Db<D> {
	pub fn new(name: &str) -> Result<Self> {
		db(name)
	}

	/// allows u to change the data inside and writes the db to the file
	pub fn mutate<R>(&mut self, f: impl FnOnce(&mut D) -> R) -> Result<R> {
		let r = f(&mut self.current);

		self.file.set_len(0).map_err(Error::TruncateWhenWrite)?;
		self.file
			.seek(std::io::SeekFrom::Start(0))
			.map_err(Error::SeekWhenWrite)?;

		let mut writer = std::io::BufWriter::new(&mut self.file);
		serde_json::to_writer(&mut writer, &self.current).map_err(Error::SerializeWrite)?;

		Ok(r)
	}
}

impl<D: Data> AsRef<D> for Db<D> {
	fn as_ref(&self) -> &D {
		&self.current
	}
}
