use actix_files::NamedFile;
use actix_web::{post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use crate::database::{ verifier};
use std::process::Command;
use std::sync::atomic::{AtomicPtr, Ordering};


pub static INFO_REVOKE: AtomicPtr<CodeCheck> = AtomicPtr::new(std::ptr::null_mut());


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct CodeCheck {
    csr: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Reason {
    reason : String,
    email: String,
}


fn revoke_crl(email:&String,code:&String,_reason:&String) -> bool {
    let path_certif = format!("usercertificate/{}/{}/certificate.crt", email,code);
    let result = Command::new("openssl")
        .arg("ca")
        .arg("-config")
        .arg("../ACI/intermediate_ca.cnf")
        .arg("-revoke")
        .arg(path_certif)
        .output();
    
    match result {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
            output.status.success()
        },
        Err(error) => {
            eprintln!("Erreur lors de la révocation: {}", error);
            false
        }
    }
}



#[post("/revoke")]
pub async fn revoker() -> Result<NamedFile, actix_web::Error> {
        let path: PathBuf = "./static/revoke.html".into();
        Ok(NamedFile::open(path)?)
}



#[post("/verify_revoke")]
pub async fn verify_revoker(code_check : web::Form<CodeCheck>) -> Result<NamedFile, actix_web::Error> {
    INFO_REVOKE.store(Box::into_raw(Box::new(code_check.clone())), Ordering::SeqCst);
    if verifier(code_check.csr.clone()) == true {
        let path: PathBuf = "./static/revocation.html".into();
        Ok(NamedFile::open(path)?)
    }
    else {
        let path: PathBuf = "./static/revoke.html".into();
        Ok(NamedFile::open(path)?)
    }
}


fn update_crl() -> bool {
    let result = Command::new("openssl")
        .arg("ca")
        .arg("-config")
        .arg("../ACI/intermediate_ca.cnf")
        .arg("-gencrl")
        .arg("-out")
        .arg("../CRL/list.crl")
        .output();
    
    match result {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
            output.status.success()
        },
        Err(error) => {
            eprintln!("Erreur lors de la mise à jour de la CRL: {}", error);
            false
        }
    }
}

#[post("/revoke_reason")]
pub async fn revoke_reason(reason: web::Form<Reason>) -> Result<NamedFile, actix_web::Error> {
    println!("email: {}", reason.email);
    println!("reason: {}", reason.reason);
    println!("csr: {}", unsafe { &INFO_REVOKE.load(Ordering::SeqCst).as_ref().unwrap().csr });
    revoke_crl(&reason.email,unsafe { &INFO_REVOKE.load(Ordering::SeqCst).as_ref().unwrap().csr },&reason.reason);
    update_crl();
        let path: PathBuf = "./static/page.html".into();
        println!("Revocation effectuée");
        Ok(NamedFile::open(path)?)
}