use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::vec::Vec;
use std::io::SeekFrom;
use std::io::Seek;
#[derive(Serialize, Deserialize,Debug,Clone)]
struct Entry {
    path: String,
    code: String,
}

pub fn ajouter(path: String, code: String)  {
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("database.json")
    .unwrap();
    let mut contents = String::new();

    // verifier si le fichier est vide si oui le remplir
    if file.read_to_string(&mut contents).unwrap() == 0 {
        let entries = vec![Entry { path, code }];
        let serialized_entries = serde_json::to_string(&entries).unwrap() + "\n";
        file.write_all(serialized_entries.as_bytes()).unwrap();
    }
    else {
        println!("File is not empty");
        let mut entries: Vec<Entry> = serde_json::from_str(&contents).unwrap();
        println!("entries: {:?}", entries);
        for entry in &mut entries {
            println!("TEST {} {}", entry.path, entry.code);
        }
        entries.push(Entry { path, code });
        println!("entries: {:?}", entries);
        let serialized_entries = serde_json::to_string(&entries).unwrap() + "\n";
        println!("serialized_entries: {}", serialized_entries);
        file.set_len(0).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();

        file.write_all(serialized_entries.as_bytes()).unwrap();
    }
    }



pub fn verifier(code: String) -> bool {
    let mut file = match File::open("database.json") {
        Ok(file) => file,
        Err(_) => return false, // si le fichier ne peut pas être ouvert, retourne false directement
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return false; // si la lecture échoue, retourne false directement
    }
    let deserialized_entries: Vec<Entry> = match serde_json::from_str(&contents) {
        Ok(entries) => entries,
        Err(_) => return false, // si la désérialisation échoue, retourne false directement
    };
    for entry in deserialized_entries {
        if entry.code == code {
            println!("Path: {}, Code: {}", entry.path, entry.code);
            return true;
        }
    }
    false
}


pub fn revoquer(code: String) -> bool {
    let mut file = File::open("database.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut entries: Vec<Entry> = serde_json::from_str(&contents).unwrap();
    let mut found = false;
    entries.retain(|entry| {
        if entry.code == code {
            found = true;
            false
        } else {
            true
        }
    });
    if found {
        let serialized_entries = serde_json::to_string(&entries).unwrap() + "\n";
        let mut file = File::create("database.json").unwrap();
        file.write_all(serialized_entries.as_bytes()).unwrap();
        true
    } else {
        println!("Not found");
        false
    }
}