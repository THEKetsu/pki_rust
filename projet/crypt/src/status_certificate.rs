
use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use std::str;
use crate::database::{verifier};
use std::process::{Command};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct CheckInfos {
    email: String,
    csr: String,
}



fn launch_oscp() -> std::process::Child {
    let result = Command::new("openssl")
        .arg("ocsp")
        .arg("-port")
        .arg("8081")
        .arg("-index")
        .arg("../ACI/intermediate_ca.db")
        .arg("-rkey")
        .arg("../ACI/intermediate_ca.key")
        .arg("-rsigner")
        .arg("../ACI/intermediate_ca.crt")
        .arg("-CA")
        .arg("../ACI/intermediate_ca.crt")
        .arg("-text")
        .arg("-crl_check")
        .spawn();
    
        match result {
            Ok(child) => {
                // Afficher un message pour confirmer que le processus est lancé
                println!("OCSP lancé en arrière-plan avec le PID {}", child.id());
                child
            },
            Err(error) => {
                eprintln!("Erreur lors du lancement de l'OCSP: {}", error);
                std::process::exit(1)
            }
        }
}


fn request_ocsp(mail : &String , code : &String)-> String {
    let path_cert = format!("usercertificate/{}/{}/certificate.crt",mail,code);
    let result = Command::new("openssl")
    .arg("ocsp")
    .arg("-issuer")
    .arg("../ACI/intermediate_ca.crt")
    .arg("-cert")
    .arg(path_cert)
    .arg("-url")
    .arg("http://localhost:8081")
    .arg("-text")
    .output();

match result {
    Ok(output) => {
        String::from_utf8_lossy(&output.stdout).to_string()
    },
    Err(error) => {
        eprintln!("Erreur lors de la révocation: {}", error);
        "Erreur lors de la révocation".to_string()
    }
    
}
}

#[post("/verify_status")]
pub async fn see_ocsp_status(code : web::Form<CheckInfos> ) -> HttpResponse {
  
    if verifier(code.csr.clone()) == false {
        return HttpResponse::Ok().body("Le certificat n'existe pas");
    }
    let mut child = launch_oscp();
    let content: String= request_ocsp(&code.email, &code.csr);
    println!("BODY_CONTENT : {}",content);
    child.kill().expect("Echec de l'arrêt de l'OCSP");
    HttpResponse::Ok().body(content)
}