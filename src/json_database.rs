pub struct User {
	uuid: String,
	salt: String,
	opslimit: String,
	memlimit: String,
	pubkey_file: Option<String>,
	privkey_file: Option<String>,
}

pub struct AttachmentMeta {
	uuid: String,
	name: String,
}

// Metadata describing an entry
pub struct EntryMeta {
	uuid: String,
	created: String,
	updated: String,
	folder: String,
	favorite: String
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
	attachments: Vec<String>,
}
