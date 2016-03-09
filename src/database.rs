extern crate sodiumoxide;
extern crate uuid;
extern crate time;

use self::sodiumoxide::crypto::pwhash;
use self::uuid::Uuid;

// Create custom wrappers for each way a UUID can be used
pub struct AttachmentUUID(uuid::UUID);
pub struct EntryUUID(uuid::UUID);
pub struct UserUUID(uuid::UUID);

pub struct User {
	uuid: UserUUID,
	salt: pwhash::Salt,
	opslimit: pwhash::OpsLimit,
	memlimit: pwhash::MemLimit,
	pubkey_file: Option<String>,
	privkey_file: Option<String>,
}

pub struct AttachmentMeta {
	uuid: AttachmentUUID,
	name: String,
}

// Metadata describing an entry
pub struct EntryMeta {
	uuid: EntryUUID,
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
	attachments: Vec<AttachmentUUID>,
}
