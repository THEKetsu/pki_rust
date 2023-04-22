use actix_files::NamedFile;
use actix_web::{post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;
use crate::database::revoquer;

lazy_static! {
    pub static ref RANDOM_NUMBER_REVOKE: i32 = thread_rng().gen_range(10000..=30000);
}


#[derive(Serialize, Deserialize, Debug)]
struct EmailCheck {
    csr: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Email {
    email: String,
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
