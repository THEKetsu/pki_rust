use actix_web::{ post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::str;
use std::fs;
use actix_files::NamedFile;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct CertificateRequest {
    email: String,
    csr: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CSRData {
    common_name: String,
    organizational_unit: String,
    locality: String,
    state: String,
    email_address: String,
}

fn generate_private_key_and_public_key() {
    let _private_key = Command::new("openssl")
    .arg("genpkey")
    .arg("-algorithm")
    .arg("RSA")
    .arg("-out")
    .arg("private.key")
    .output()
    .expect("Erreur lors de la génération de la clé privée");
}

fn generate_certificate(info1_: web::Form<CSRData>){
     // Exécuter la commande pour générer la CSR  
    let information = format!("/C=FR/ST={}/L={}/O=Isen/OU={}/CN={}/emailAddress={}", info1_.state, info1_.locality, info1_.organizational_unit, info1_.common_name, info1_.email_address);
    println!("{:?}",information);
     let _csr = Command::new("openssl") 
        .arg("req")
        .arg("-new")
        .arg("-key")
        .arg("private.key")
        .arg("-out")
        .arg("csr.csr")
        .arg("-subj")
        .arg(information)
        .output()
        .expect("Erreur lors de la génération de la CSR");
}


fn verify_certificate() -> bool {
    let result = Command::new("openssl")
        .arg("req")
        .arg("-text")
        .arg("-noout")
        .arg("-in")
        .arg("csr.csr")
        .output();
    
    match result {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            true
        },
        Err(error) => {
            eprintln!("Erreur lors de la génération de la CSR : {}", error);
            false
        }
    }
}

fn signed_certificate() -> bool {
    println!("Generating certificate...");
    let result = Command::new("openssl")
        .arg("x509")
        .arg("-req")
        .arg("-in")
        .arg("csr.csr")
        .arg("-CA")
        .arg("../ACI/intermediate.crt")
        .arg("-CAkey")
        .arg("../ACI/intermediate.key")
        .arg("-CAcreateserial")
        .arg("-out")
        .arg("server.crt")
        .arg("-days")
        .arg("365")
        .arg("-sha256")
        .output();
    println!("certificate CREATED...");
    match result {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                true
            } else {
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
                false
            }
        },
        Err(error) => {
            eprintln!("Error: {}", error);
            false
        }
    }
}



#[post("/keys")]
pub async fn generate_csr(info_: web::Form<CSRData>) -> Result<NamedFile, actix_web::Error> {
    let _path: PathBuf = "./static/envoyer_mail.html".into();
    println!("{:?}",info_);
    // On mettra l'info après je veux juste voir la geule de info 
    generate_private_key_and_public_key(); // Exécuter la commande pour générer la clé privée
    generate_certificate(info_); // Exécuter la commande pour générer la CSR
    signed_certificate();
    let crt_content = fs::read_to_string("server.crt").unwrap();
    if   verify_certificate() == true && signed_certificate() == true{
        HttpResponse::Ok().body("Vérification CSR NOT");
    } else {
        HttpResponse::Ok().body("Vérification CSR NOT");
    }
    let _path: PathBuf = "./static/download.html".into();
        Ok(NamedFile::open(_path)?) 
}