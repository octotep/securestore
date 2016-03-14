extern crate sodiumoxide;
extern crate uuid;
extern crate time;

use self::sodiumoxide::crypto::pwhash;
use self::uuid::Uuid;

// Create custom wrappers for each way a UUID can be used
pub struct AttachmentUuid(uuid::Uuid);
pub struct EntryUuid(uuid::Uuid);
pub struct UserUuid(uuid::Uuid);

pub struct User {
	uuid: UserUuid,
	salt: pwhash::Salt,
	opslimit: pwhash::OpsLimit,
	memlimit: pwhash::MemLimit,
	pubkey_file: Option<String>,
	privkey_file: Option<String>,
}

pub struct AttachmentMeta {
	uuid: AttachmentUuid,
	name: String,
}

// Metadata describing an entry
pub struct EntryMeta {
	uuid: EntryUuid,
	created: time::Timespec,
	updated: time::Timespec,
	name: String,
}

// The contents of a custom field for an entry
pub struct EntryField {
	name: String,
	value: String,
}

// A single entry in the database
pub struct Entry {
	website: String,
	username: String,
	password: String,
	notes: String,
	custom_fields: Vec<EntryField>,
	attachments: Vec<AttachmentUuid>,
}

//pub fn create(path: Path) {
//	let mut new_path = path.clone();
//	new_path.push("securestore/users");
//	DirBuilder::new()
//	    .recursive(true)
//	    .create(new_path).unwrap();
//	Database {
//		version: DB_VERSION,
//		path: new_path,
//	}
//}
