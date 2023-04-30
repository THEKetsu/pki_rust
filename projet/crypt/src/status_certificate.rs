
use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use std::str;
use std::process::Command;
use crate::database::{verifier};
use std::thread;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct CheckInfos {
    email: String,
    csr: String,
}



fn launch_oscp() -> bool {
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
    let ocsp_thread = thread::spawn(|| {
        launch_oscp();
    });
     let content: String= request_ocsp(&code.email, &code.csr);
    ocsp_thread.join().unwrap();
    println!("BODY_CONTENT : {}",content);
    HttpResponse::Ok().body(content)
}