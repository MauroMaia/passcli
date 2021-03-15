use kpdb::{CompositeKey, Database};
use std::fs::File;
use std::path::Path;
//see interface to kdbx @ https://github.com/sru-systems/rust-kpdb

pub fn create_file_with_password(path: &str, password: &str) -> Result<String, String> {
    // FIXME - check if destination has write permissions
    if Path::new(path).exists() {
        return Err(format!("File {} already exist! ", path));
    } else if "".eq(password) {
        return Err("Password is empty!".to_string());
    }

    // Create a new database.
    let key = CompositeKey::from_password(password);
    let db = Database::new(&key);

    let mut file = File::create(path).unwrap();
    db.save(&mut file).unwrap();
    Ok("".parse().unwrap())
}

/*
pub fn create_file_with_keyfile(path: &str, file: &str) {

    // Create a new database.
    let key = CompositeKey::from_key_file("password");
    let mut db = Database::new(&key);

    let mut file = File::create("/tmp/test_db_with_password.kdbx").unwrap();
    db.save(&mut file).unwrap();
}
*/
