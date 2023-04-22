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


lazy_static! {
    static ref RANDOM_NUMBER: i32 = thread_rng().gen_range(10000..=30000);
}



#[get("/")]
async fn index() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./static/main.html".into();
    Ok(NamedFile::open(path)?)
}



/* 

#[post("/verify")]
async fn verify(info: web::Form<CertificateRequest>) -> HttpResponse {
    //J'arrive pas à print les info json de la page html
    println!("Received request {:?}", info);
    let code = &info.csr;
    let verif = *RANDOM_NUMBER;
    let veristr= verif.to_string();
    if  code == &veristr {
        HttpResponse::Ok().body("Code correct")
    } else {
        HttpResponse::Ok().body("Code incorrect")
    }
}
*/
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new()
        .service(index)
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
