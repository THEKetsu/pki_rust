use actix_files::NamedFile;
use actix_web::{post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;
use crate::database::{revoquer, verifier};
use std::process::Command;

lazy_static! {
    pub static ref RANDOM_NUMBER_REVOKE: i32 = thread_rng().gen_range(10000..=30000);
}


#[derive(Serialize, Deserialize, Debug)]
struct EmailCheck {
    csr: String,
}




fn revoke_ocsp(email:&String) -> bool {
    let result = Command::new("openssl")
        .arg("ocsp")
        .arg("-port")
        .arg("8888")
        .arg("-index")
        .arg("OCSP/index.txt")
        .arg("-rsigner ")
        .arg("OCSP/ocsp.crt")
        .arg("-rkey")
        .arg("OCSP/ocsp.key")
        .arg("-CA")
        .arg("ACI/intermediate_ca.crt")
        .arg("-ndays")
        .arg("365")
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



#[post("/revoke")]
pub async fn revoker(info1_: web::Form<EmailCheck>) -> Result<NamedFile, actix_web::Error> {
    if false == revoquer(info1_.csr.clone()) {
        let path: PathBuf = "./static/revoke.html".into();
        Ok(NamedFile::open(path)?)
    }
    else{
        println !("Revoked");
        let path: PathBuf = "./static/page.html".into();
        Ok(NamedFile::open(path)?)
    }
}


