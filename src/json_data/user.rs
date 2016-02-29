pub struct User {
	uuid: String,
	salt: String,
	opslimit: String,
	memlimit: String,
	pubkey_file: Option<String>,
	privkey_file: Option<String>,
}
