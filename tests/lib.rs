extern crate securestore;

use std::path::Path;

#[test]
fn it_works() {
	securestore::init();
	let db = securestore::Database::new(Path::new("."));
	securestore::User::new(&db, "testuser", "testpassword");
	assert!(true);
}
