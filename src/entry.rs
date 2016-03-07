extern crate uuid;
extern crate time;

use self::uuid::Uuid;

pub struct Field {
	name: String,
	value: String,
}

pub struct Entry {
	uuid: uuid::Uuid,
	created: time::Timespec,
	updated: time::Timespec,
	name: String,
	website: String,
	username: String,
	password: String,
	notes: String,
	custom_fields: Vec<Field>,
	attachments: Vec<Uuid>,
}
