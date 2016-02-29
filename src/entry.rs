pub struct Field {
	name: String,
	value: String,
}

pub struct Entry {
	uuid: String,
	name: String,
	website: String,
	username: String,
	password: String,
	notes: String,
	custom_fields: Vec<Field>,
	attachments: Vec<String>,
}
