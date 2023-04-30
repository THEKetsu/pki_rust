use actix_files::NamedFile;
use actix_web::{get, App,HttpServer};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
mod generate_csr;
mod mailing;
mod download;
mod revoke;
mod database;
mod status_certificate;

#[derive(Serialize, Deserialize, Debug)]
struct CertificateRequest {
    email: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct CSRData {
    common_name: String,
    organization_name: String,
    organizational_unit: String,
    locality: String,
    state: String,
    country: String,
    pub email_address: String,
}

#[get("/")]
async fn index() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./static/page.html".into();
    Ok(NamedFile::open(path)?)
}


#[get("/generate-certif")]
async fn index_2() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./static/mail.html".into();
    Ok(NamedFile::open(path)?)
}

#[get("/revoke")]
async fn index_3() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./static/revoke.html".into();
    Ok(NamedFile::open(path)?)
}

#[get("/see_certificate")]
async fn index_4() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./static/verif.html".into();
    Ok(NamedFile::open(path)?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .service(index)
        .service(index_2)
        .service(index_3)
        .service(index_4)
        .service(generate_csr::generate_csr)
        .service(mailing::mail_send)
        .service(mailing::check_code)
        .service(download::download_file)
        .service(mailing::check_code)
        .service(revoke::revoker)
        .service(revoke::verify_revoker)
        .service(revoke::revoke_reason)
        .service(status_certificate::see_ocsp_status)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
