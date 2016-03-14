extern crate rustc_serialize;
extern crate sodiumoxide;

use sodiumoxide::crypto::pwhash;
use rustc_serialize::json;
use rustc_serialize::hex::ToHex;

use std::path::{Path, PathBuf};
use std::fs::{self, DirBuilder};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

pub mod database;

static DB_VERSION: &'static str = "0.1";

#[derive(RustcDecodable, RustcEncodable)]
pub struct Database  {
	version: String,
	path: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
	username: String,
	salt: String,
	opslimit: usize,
	memlimit: usize,
	pwhash: String,
}

// Initializs libsodium
pub fn init() {
	sodiumoxide::init();
}

// TODO: change to open an existing db, and decide what a valid database should look like
impl Database {
	pub fn new(path: &Path) -> Database {
		let mut new_path = path.to_path_buf();
		new_path.push("securestore");
		new_path.push("users");
		DirBuilder::new()
			.recursive(true)
			.create(new_path.as_path()).unwrap();
		let db = Database {
			version: DB_VERSION.to_string(),
			path: new_path.to_str().unwrap().to_string(),
		};
		println!("{}", json::encode(&db).unwrap());
		new_path.push("../dbinfo.json");
		let mut db_info_file = File::create(new_path).unwrap();
		db_info_file.write_all(json::encode(&db).unwrap().as_bytes());

		db
	}
}

impl User {
	// TODO: determine if user hash is even necessary, if so, HASH IT
	pub fn new(db: &Database, username: &str, password: &str) -> User {
		let mut userpath = PathBuf::from(&db.path);
		userpath.push(username);
		DirBuilder::new()
			.recursive(true)
			.create(&userpath).unwrap();
		let usersalt = pwhash::gen_salt();
		println!("Salt: {:?}", &usersalt.0.to_hex());
		let user = User {
			username: username.to_owned(),
			salt: usersalt.0.to_hex(),
			opslimit: pwhash::OPSLIMIT_INTERACTIVE.0,
			memlimit: pwhash::MEMLIMIT_INTERACTIVE.0,
			pwhash: password.to_owned(),
		};
		userpath.push("userinfo.json");
		let mut user_info_file = File::create(userpath).unwrap();
		user_info_file.write_all(json::encode(&user).unwrap().as_bytes());
		user
	}
}
