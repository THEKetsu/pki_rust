use actix_files::NamedFile;
use actix_web::{post, web};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicPtr, Ordering};
use crate::database::verifier;
pub static INFO_EMAIL: AtomicPtr<Email> = AtomicPtr::new(std::ptr::null_mut());

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Email {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EmailCheck {
    csr: String,
}

// fonction qui génère un nombre aléatoire entre 10000 et 30000 et qui vérif que le code n'est pas déjà utilisé
pub fn generate_code() -> i32 {
    let code = thread_rng().gen_range(10000..=30000);
    while verifier(code.to_string()) == true {
        let code = thread_rng().gen_range(10000..=30000);
    }
    code
}











lazy_static! {
    pub static ref RANDOM_NUMBER: i32 = generate_code();
    //faire une boucle pour vérifer que le code n'est pas déjà utilisé
}


//Donne un autre nom de la fonction 

#[post("/mail")]
pub async fn mail_send(info2_: web::Form<Email>) -> Result<NamedFile, actix_web::Error> {
    let code = *RANDOM_NUMBER;
    let chaine = "Votre code est :";
    INFO_EMAIL.store(Box::into_raw(Box::new(info2_.clone())), Ordering::SeqCst);
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

#[post("/verify")]
 async fn check_code(info1_: web::Form<EmailCheck>) -> Result<NamedFile, actix_web::Error> {
    //J'arrive pas à print les info json de la page html
    let code = &info1_.csr;
    let verif = *RANDOM_NUMBER;
    let veristr= verif.to_string();
    if  code == &veristr {
        let path: PathBuf = "./static/formulaire.html".into();
        println!("Code CHECK !");
        Ok(NamedFile::open(path)?) 
    } 
    else {
        let path_error: PathBuf = "./static/mail.html".into();
        println!("Code FAIL !");
        Ok(NamedFile::open(path_error)?)
    }    
}

