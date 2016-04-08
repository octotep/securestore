extern crate securestore;

use std::path::Path;

#[test]
fn it_works() {
	securestore::init();
	let db = securestore::Database::new(Path::new("."), "testpassword");
	securestore::Entry::new(&db, "My YCP", "my.ycp.edu", "college", true, "cyealy", "notmypassword", "This is a multiline comment\nHopefully");
	assert!(true);
}
