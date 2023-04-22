use actix_files::NamedFile;
use actix_web::{post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;

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


#[post("/verify_revoke")]
pub async fn check_code(info1_: web::Form<EmailCheck>) -> Result<NamedFile, actix_web::Error> {
    //J'arrive pas à print les info json de la page html
    let code = &info1_.csr;
    let verif = *RANDOM_NUMBER_REVOKE;
    let veristr= verif.to_string();
    if  code == &veristr {
        let path: PathBuf = "./static/index.html".into();
        Ok(NamedFile::open(path)?) 
    } 
    else {
        let path_error: PathBuf = "./static/main.html".into();
        Ok(NamedFile::open(path_error)?)
    }    
}


#[post("/mail")]
pub async fn mail_send_revoke(info2_: web::Form<Email>) -> Result<NamedFile, actix_web::Error> {
    let code = *RANDOM_NUMBER_REVOKE;
    let chaine = "Votre code est :";
    let chaine_f = format!("{} {}", chaine, code.to_string());
    let email = Message::builder()
        .from("cryptoISEN30@gmail.com".parse().unwrap())
        .to(info2_.email.parse().unwrap())
        .subject("Vérification no-reply")
        .body(chaine_f)
        .unwrap();

    //Défini le serveur smtp mail
    let  mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(Credentials::new("cryptoisen30".into(), "htov vmlv vdkz vpor".into()))
        .build();
                
     // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    let path: PathBuf = "./static/code.html".into();
    Ok(NamedFile::open(path)?) 
}