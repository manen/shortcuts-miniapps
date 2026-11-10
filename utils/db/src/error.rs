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
	#[error("failed to read metadata for db file or folder\n{0}")]
	Metadata(std::io::Error),

	#[error("failed to deserialize and/or read data inside db\n{0}")]
	DeserializeRead(serde_json::error::Error),
	#[error("failed to serialize and/or write data inside db\n{0}")]
	SerializeWrite(serde_json::error::Error),

	#[error("failed to truncate old db file when writing new contents (set_len failed)\n{0}")]
	TruncateWhenWrite(std::io::Error),
	#[error("failed to seek to beginning of db file when writing changed contents\n{0}")]
	SeekWhenWrite(std::io::Error),

	// list
	#[error("this folder doesn't exist so it can't be listed")]
	FolderDoesntExit { dir_supplied: String },
	#[error("readdir failed on a specific element\n{0}")]
	ReadDirWhileReading(std::io::Error),
}
pub type Result<T, E = Error> = std::result::Result<T, E>;
