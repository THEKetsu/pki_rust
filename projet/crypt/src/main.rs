use actix_files::NamedFile;
use actix_web::{get, App,HttpServer};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;
mod generate_csr;
mod mailing;
mod download;
mod revoke;
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
    email_address: String,
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

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .service(index)
        .service(index_2)
        .service(index_3)
        .service(generate_csr::generate_csr)
        .service(mailing::mail_send)
        .service(mailing::check_code)
        .service(download::download_file)
        .service(mailing::check_code)
        .service(revoke::mail_send_revoke)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
