use actix_web::{ get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::fs;
use crate::generate_csr::INFO;
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Serialize, Deserialize, Debug)]
struct CertificateRequest {
    email: String,
    csr: String,
}


#[get("/download")]
pub async fn download_file() -> impl Responder {
    let info = unsafe { INFO.load(Ordering::SeqCst).as_ref().unwrap() };
    let file_path = format!("usercertificate/{}/server.crt",info.email_address);
    let file_path_copy = file_path.clone();
    // Vérifier si le fichier existe
    if !fs::metadata(file_path).is_ok() {
        return HttpResponse::NotFound().body(format!("Le fichier {} n'existe pas.", file_path_copy));
    }
    // Définir le nom de fichier à télécharger
    let file_name = "server.crt";
    // Lire le contenu du fichier
    let file_content = fs::read(file_path_copy).unwrap();

    // Créer une réponse HTTP avec le contenu du fichier en tant que corps de réponse
    let mut response = HttpResponse::Ok().body(file_content);

    // Définir l'en-tête Content-Disposition pour télécharger le fichier avec un nom de fichier spécifique
    response.headers_mut().insert(
        actix_web::http::header::CONTENT_DISPOSITION,
        actix_web::http::header::HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_name)).unwrap(),
    );

    // Retourner la réponse HTTP
    response
}
