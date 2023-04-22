use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::vec::Vec;
use std::io::{BufReader, BufWriter};
#[derive(Serialize, Deserialize,Debug,Clone)]
struct Entry {
    path: String,
    code: String,
}

pub fn ajouter(path: String, code: String) -> std::io::Result<()> {
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
    let reader = BufReader::new(file);
    let  my_array: Vec<Entry> = serde_json::from_reader(reader)?;
      // Ajouter la nouvelle entrée au tableau
      let file = File::create("database.json")?;
      let writer = BufWriter::new(file);
      serde_json::to_writer(writer, &my_array)?;
      println!("{:}",writer.to_string());
      // Fermer le fichier ouvert
      Ok(())
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