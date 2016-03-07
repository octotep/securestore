pub struct User {
	uuid: String,
	salt: String,
	opslimit: usize,
	memlimit: usize,
	pubkey_file: Option<String>,
	privkey_file: Option<String>,
}
