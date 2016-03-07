extern crate sodiumoxide;
extern crate uuid;

use self::sodiumoxide::crypto::pwhash;
use self::uuid::Uuid;

pub struct User {
	uuid: uuid::Uuid,
	salt: pwhash::Salt,
	opslimit: pwhash::OpsLimit,
	memlimit: pwhash::MemLimit,
	pubkey_file: Option<String>,
	privkey_file: Option<String>,
}
