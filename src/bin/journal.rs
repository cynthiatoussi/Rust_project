use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::sync::Arc;

use chrono::Utc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

/// Chemin du fichier de log.
const LOG_FILE_PATH: &str = "logs/server.log";

/// Lance le serveur TCP et gère les connexions entrantes.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Crée le dossier `logs/` s’il n’existe pas.
    create_dir_all("logs")?;

    // Ouvre le fichier en mode append (ajout).
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE_PATH)?;

    // Partage du fichier entre tâches via Arc<Mutex<_>>.
    let shared_file = Arc::new(Mutex::new(file));

    // Écoute des connexions sur le port TCP 8080.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("[OK] Serveur lancé sur 127.0.0.1:8080");

    // Boucle principale du serveur : accepte les connexions.
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("[INFO] Nouvelle connexion : {}", addr);

        let file_clone = Arc::clone(&shared_file);

        // Lancement d'une tâche asynchrone indépendante.
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr, file_clone).await {
                eprintln!("[ERREUR] Connexion {} : {}", addr, e);
            }
        });
    }
}

/// Gère la communication avec un client.
async fn handle_client(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    log_file: Arc<Mutex<std::fs::File>>,
) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    // Lecture ligne par ligne envoyée par le client.
    while let Some(line) = lines.next_line().await? {
        let timestamp = Utc::now().to_rfc3339();
        let log_line = format!("[{}]  {}\n", timestamp, line);

        {
            // Bloc limité : évite de bloquer trop longtemps le verrou.
            let mut file = log_file.lock().await;
            file.write_all(log_line.as_bytes())?;
            file.flush()?;
        }

        println!("[LOG {}] {}", addr, line);
    }

    println!("[INFO] Déconnexion : {}", addr);
    Ok(())
}
