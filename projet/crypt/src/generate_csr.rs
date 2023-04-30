use actix_web::{ post, web};
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::str;
use std::fs;
use actix_files::NamedFile;
use std::path::PathBuf;
use std::sync::atomic::{AtomicPtr, Ordering};
use crate::mailing::INFO_EMAIL;
pub static INFO: AtomicPtr<CSRData> = AtomicPtr::new(std::ptr::null_mut());
use crate::mailing::RANDOM_NUMBER;
use crate::database::ajouter;

#[derive(Serialize, Deserialize, Debug)]
struct CertificateRequest {
    email: String,
    csr: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
 pub  struct CSRData {
    common_name: String,
    organizational_unit: String,
    locality: String,
    state: String,
    pub email_address: String,
}

fn generate_private_key_and_public_key(email:&String) {
    let path_private : String =  format!("usercertificate/{}/{}/private.key",email,RANDOM_NUMBER.to_string());
    let _private_key = Command::new("openssl")
    .arg("ecparam")
    .arg("-name")
    .arg("prime256v1")
    .arg("-genkey")
    .arg("-noout")
    .arg("-out")
    .arg(path_private)
    .output()
    .expect("Erreur lors de la génération de la clé privée");
}

fn generate_certificate(info1_: CSRData){
     // Exécuter la commande pour générer la CSR  
    let path_private_key : String =  format!("usercertificate/{}/{}/private.key",info1_.email_address,RANDOM_NUMBER.to_string());
    let path_csr : String =  format!("usercertificate/{}/{}/csr.csr",info1_.email_address,RANDOM_NUMBER.to_string());
    let information = format!("/C=FR/ST={}/L={}/O=Isen/OU={}/CN={}/emailAddress={}", info1_.state, info1_.locality, info1_.organizational_unit, info1_.common_name, info1_.email_address);
    println!("{:?}",information);
     let _csr = Command::new("openssl") 
        .arg("req")
        .arg("-new")
        .arg("-key")
        .arg(path_private_key)
        .arg("-out")
        .arg(path_csr)
        .arg("-sha384")
        .arg("-subj")
        .arg(information)
        .output()
        .expect("Erreur lors de la génération de la CSR");
}


fn verify_certificate(email:&String) -> bool {
    let path_verif = format!("usercertificate/{}/{}/csr.csr",email,RANDOM_NUMBER.to_string());
    let result = Command::new("openssl")
        .arg("req")
        .arg("-text")
        .arg("-noout")
        .arg("-in")
        .arg(path_verif)
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

fn signed_certificate(email:&String) -> bool {
    println!("Generating certificate...");
    let path_csr = format!("usercertificate/{}/{}/csr.csr",email,RANDOM_NUMBER.to_string());
    let path_crt = format!("usercertificate/{}/{}/certificate.crt",email,RANDOM_NUMBER.to_string());
    let result = Command::new("openssl")
        .arg("x509")
        .arg("-req")
        .arg("-in")
        .arg(path_csr)
        .arg("-CA")
        .arg("../ACI/intermediate_ca.crt")
        .arg("-CAkey")
        .arg("../ACI/intermediate_ca.key")
        .arg("-CAcreateserial")
        .arg("-out")
        .arg(path_crt)
        .arg("-days")
        .arg("365")
        .arg("-sha384")
        .arg("-passin") // ajouter cette ligne pour spécifier le mot de passe pour l'option -CAkey
        .arg("pass:isen") 
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
    INFO.store(Box::into_raw(Box::new(info_.clone())), Ordering::SeqCst);
    let email_from_mailing = unsafe{ INFO_EMAIL.load(Ordering::SeqCst).as_ref().unwrap()};
    if info_.email_address != email_from_mailing.email{
        println!("Erreur d'adresse email");
        let _path: PathBuf = "./static/page.html".into();
        return Ok(NamedFile::open(_path)?);
    }
    // check si le dossier existe
    let directory_name = format!("usercertificate/{}/{}",email_from_mailing.email.clone(),RANDOM_NUMBER.to_string());
    match fs::create_dir_all(directory_name) {
        Ok(_) => println!("Directory created successfully"),
        Err(error) => println!("Error creating directory: {}", error),
    }
    println!("{:?}",info_);
    // On mettra l'info après je veux juste voir la geule de info 
    generate_private_key_and_public_key(&email_from_mailing.email.clone()); // Exécuter la commande pour générer la clé privée
    generate_certificate(info_.clone()); // Exécuter la commande pour générer la CSR
    signed_certificate(&email_from_mailing.email.clone());
    let path_verif = format!("usercertificate/{}/{}/certificate.crt",email_from_mailing.email.clone(),RANDOM_NUMBER.to_string());
    let _crt_content = fs::read_to_string(path_verif).unwrap();
    if verify_certificate(&email_from_mailing.email.clone()) {
        println!("Vérification CSR OK");
        let random_number = RANDOM_NUMBER.to_string();
        ajouter(email_from_mailing.email.clone(),random_number);
    } else {
        println!("Vérification CSR NOT");
    }
    let _path: PathBuf = "./static/download.html".into();
    Ok(NamedFile::open(_path)?)
}
