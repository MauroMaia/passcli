use kpdb::{CompositeKey, Database};
use std::fs::{File, OpenOptions};
use std::path::Path;

#[derive(serde_derive::Serialize, Debug)]
pub struct Entry<'a> {
    pub title: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub url: &'a str,
    pub group: &'a str,
}

pub fn add_entry(
    vault_path: &str,
    vault_password: &str,
    new_entry: &Entry,
) -> Result<String, String> {
    return open_file_connection_with_password(vault_path, vault_password)
        .and_then(|mut db| {
            //TODO - check if entry already exists
            //let entries = db.find_entries("proton");
            //assert_eq!(entries.len(), 0);

            // Create an entry for ProtonMail and add it to the Email group.
            let mut entry = kpdb::Entry::new();
            entry.set_title(new_entry.title);
            entry.set_username(new_entry.username);
            entry.set_password(new_entry.password);
            entry.set_url(new_entry.url);

            db.root_group.add_entry(entry);

            let mut filenew = File::create(vault_path).unwrap();

            return db
                .save(&mut filenew)
                .and_then(|_| {
                    list_all_entries(vault_path, vault_password);
                    return Ok("".parse().unwrap());
                })
                .or_else(|err| {
                    return Err(format!("Error: {}", err));
                });
        })
        .or_else(|err| {
            return Err(format!("Error: {}", err));
        });
}

pub fn list_all_entries(vault_path: &str, vault_password: &str) {
    let mut db = open_file_connection_with_password(vault_path, vault_password).unwrap();

    recursive_list_entries(&db.root_group, "/");
}

// ------------------------ Private Section ------------------------------------
fn recursive_list_entries(group: &kpdb::Group, path: &str) {
    for g in group.groups.iter() {
        recursive_list_entries(&g, &*(path.to_owned() + g.name.as_str() + "/"))
    }
    for i in group.entries.iter() {
        print_entry_as_json(&i, path);
    }
}

fn open_file_connection_with_password(
    vault_path: &str,
    vault_password: &str,
) -> Result<kpdb::Database, String> {
    // FIXME - check if destination has read/write permissions
    if !Path::new(vault_path).exists() {
        return Err(format!("File {} does not exist! ", vault_path));
    } else if "".eq(vault_password) {
        return Err("Password is empty!".to_string());
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(vault_path)
        .unwrap();

    let key = CompositeKey::from_password(vault_password);
    return Ok(Database::open(&mut file, &key).unwrap());
}

fn print_entry_as_json(new_entry: &kpdb::Entry, path: &str) {
    println!(
        "{}",
        serde_json::to_string(&Entry {
            title: new_entry.title().unwrap_or(""),
            username: new_entry.username().unwrap_or(""),
            password: new_entry.password().unwrap_or(""),
            url: new_entry.url().unwrap_or(""),
            group: path
        })
        .unwrap()
    )
}
