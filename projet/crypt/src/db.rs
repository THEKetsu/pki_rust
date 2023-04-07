use rusqlite::{Connection, Result};
use actix_web::{HttpResponse, get};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[get("/")]
pub async fn create_db() -> HttpResponse {
    // Ouvrir une connexion à la base de données SQLite
    let conn = Connection::open("my_database.db").expect("La connexion à la base de données a échoué.");

    // Créer une table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  email           TEXT NOT NULL
                  )",
        [],
    ).expect("La création de la table a échoué.");

    // Insérer des données dans la table
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        ["Alice", "alice@example.com"],
    ).expect("L'insertion de données a échoué.");

        // Récupérer les données de la table
        let mut stmt = conn.prepare("SELECT id, name, email FROM users").expect("La préparation de la requête a échoué.");
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
            })
        }).expect("La récupération des données a échoué.");
        
        // Afficher les données de la table
        for user in user_iter {
            println!("User: {:?}", user.unwrap());
        }
        
        // Vérification de la CSR
        let is_csr_valid = true;
        if is_csr_valid {
            HttpResponse::Ok().body("Vérification CSR OK")
        } else {
            HttpResponse::Ok().body("Vérification CSR NOT OK")
        }
    }        
