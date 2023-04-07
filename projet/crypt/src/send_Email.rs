/*use actix_files::NamedFile;
use actix_web::{get, post,App,HttpServer};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::str;
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use lettre::Message;
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
// ...



#[derive(Serialize, Deserialize, Debug)]
struct ContentEmail {
    destinaire : String,
    sujet : String,
    contenu : String,
}


#post[("/send_email")]
pub async fn email_send_sign(content:web::Form<ContentEmail>) -> Result<NamedFile, actix_web::Error> {
    // Générer la paire de clés publique/privée
    let private_key = Rsa::generate(2048).unwrap();
    let public_key = private_key.public_key_to_pem().unwrap();
    let private_key = PKey::from_rsa(private_key).unwrap();
    
    // Signer le contenu de l'e-mail avec la clé privée
    let mut signer = Signer::new(
        openssl::hash::MessageDigest::sha256(),
        &private_key
    ).unwrap();
    signer.update(email_content)?;
    let signature = signer.sign_to_vec()?;
    
    // Ajouter la signature à l'e-mail
    let email_with_signature = format!("{}\n\n--\nSignature: {}", String::from_utf8_lossy(email_content), base64::encode(&signature));
    
    // Créer un objet de type Message avec l'e-mail signé
    let email = Message::builder()
        .from("bfrost@mailfence.com".parse().unwrap())
        .to(info2_.email.parse().unwrap())
        .subject("Vérification no-reply")
        .body(email_with_signature)
        .unwrap();
    
    // Envoyer l'e-mail signé via le serveur SMTP
    let mailer = SmtpTransport::relay("smtp.mailfence.com")
        .unwrap()
        .credentials(Credentials::new("bfrost".into(), "Chaton83000!".into()))
        .build();
    
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    let path: PathBuf = "./static/index.html".into();
    Ok(NamedFile::open(path)?)
*/