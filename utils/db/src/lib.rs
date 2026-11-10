use std::{
	fs::{File, TryLockError},
	io::Seek,
	path::PathBuf,
};

use serde::{Serialize, de::DeserializeOwned};

mod open;
pub use open::*;
mod list;
pub use list::*;
mod error;
pub use error::*;

/// ---

pub trait Data: Serialize + DeserializeOwned + Default {}
impl<T> Data for T where T: Serialize + DeserializeOwned + Default {}

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
