#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate sodiumoxide;

use sodiumoxide::crypto::pwhash;
use rustc_serialize::hex::ToHex;
use uuid::Uuid;

use std::path::{Path, PathBuf};
use std::fs::{self, DirBuilder, File};
use std::error::Error;
use std::io::prelude::*;

pub mod database;

static DB_VERSION: &'static str = "0.1";

#[derive(Serialize, Deserialize)]
pub struct Database  {
	uuid: uuid::Uuid,
	version: String,
	#[serde(skip_serializing)]
	path: String,
	salt: String,
	opslimit: usize,
	memlimit: usize,
	#[serde(skip_serializing)]
	pwhash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
	uuid: uuid::Uuid,
	name: String,
	website: String,
	folder: String,
	favorite: bool,
	username: String,
	password: String,
	comments: String,
}

// Initializs libsodium
pub fn init() {
	sodiumoxide::init();
}

// TODO: change to open an existing db, and decide what a valid database should look like
// TODO: dont' store the password if we don't have to, if we do have to, then HASH IT
impl Database {
	pub fn new(path: &Path, password: &str) -> Self {
		let mut new_path = path.to_path_buf();
	    new_path.push("entries");
		DirBuilder::new()
			.recursive(true)
			.create(new_path.as_path()).unwrap();
		let usersalt = pwhash::gen_salt();
		new_path.push("..");
		let db = Database {
			version: DB_VERSION.to_string(),
			uuid: Uuid::new_v4(),
			path: new_path.to_str().unwrap().to_string(),
			salt: usersalt.0.to_hex(),
			opslimit: pwhash::OPSLIMIT_INTERACTIVE.0,
			memlimit: pwhash::MEMLIMIT_INTERACTIVE.0,
			pwhash: password.to_owned(),
		};
		println!("{}", serde_json::to_string(&db).unwrap());
		new_path.push("dbinfo.json");
		let mut db_info_file = File::create(new_path).unwrap();
		db_info_file.write_all(serde_json::to_string(&db).unwrap().as_bytes());

		db
	}

	pub fn open(path: &Path) -> Self {
		let mut dbinfo = File::open(path).unwrap();
		let mut dbjson = String::new();
		dbinfo.read_to_string(&mut dbjson).unwrap();
		let mut db: Database = serde_json::from_str(&dbjson).unwrap();
		db.path = path.to_str().unwrap().to_owned();
		let db = db; // Make db immutable
		db
	}
}

impl Entry {
	pub fn new(db: &Database, name: &str, website: &str, folder: &str, favorite: bool, username: &str, password: &str, comments: &str) -> Self {
		let entryUuid = Uuid::new_v4();
		let mut entrypath = PathBuf::from(&db.path);
		entrypath.push("entries");
		entrypath.push(entryUuid.hyphenated().to_string());
		DirBuilder::new()
			.recursive(true)
			.create(&entrypath).unwrap();
		let entry = Entry {
			uuid: entryUuid,
			name: name.to_owned(),
			website: website.to_owned(),
			folder: folder.to_owned(),
			favorite: favorite.to_owned(),
			username: username.to_owned(),
			password: password.to_owned(),
			comments: comments.to_owned(),
		};
		entrypath.push("entry.json");
		let mut entry_file = File::create(entrypath).unwrap();
		entry_file.write_all(serde_json::to_string(&entry).unwrap().as_bytes());

		entry
	}
}
