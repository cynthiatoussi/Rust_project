use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::sync::Arc;

use chrono::Utc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

const LOG_PATH: &str = "logs/server.log";

/// Simulation d’une tâche (exemple pédagogique)
async fn task(nom: &str, duree: u64) -> String {
    println!("Début de la tâche : {}", nom);
    sleep(Duration::from_secs(duree)).await;
    println!("Fin de la tâche : {}", nom);
    format!("Résultat de {}", nom)
}

/// Fonction principale asynchrone
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let debut = std::time::Instant::now();

    println!("[INFO] Lancement du serveur de journalisation...");

    // Création du dossier logs/
    create_dir_all("logs")?;

    // Ouverture (ou création) du fichier en mode ajout
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)?;

    // Partage protégé du fichier entre tâches clientes
    let log = Arc::new(Mutex::new(file));

    // Démarrage du serveur TCP
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("[OK] Serveur actif sur 127.0.0.1:8080");

    // Simulation pédagogique de 3 tâches parallèles (comme dans ton exemple)
    let t1 = tokio::spawn(task("Initialisation 1", 2));
    let t2 = tokio::spawn(task("Initialisation 2", 2));
    let t3 = tokio::spawn(task("Initialisation 3", 2));
    let _ = tokio::join!(t1, t2, t3);

    println!("[INFO] Initialisation terminée en {:?}", debut.elapsed());

    // Boucle principale d’écoute
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("[INFO] Connexion entrante : {}", addr);

        let log_clone = Arc::clone(&log);
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr, log_clone).await {
                eprintln!("[ERREUR] Client {} : {}", addr, e);
            }
        });
    }
}

/// Gère un client connecté
async fn handle_client(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    log_file: Arc<Mutex<std::fs::File>>,
) -> std::io::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    // Lecture ligne par ligne envoyée par le client
    while let Some(line) = lines.next_line().await? {
        let timestamp = Utc::now().to_rfc3339();
        let entry = format!("[{}]  {}\n", timestamp, line);

        // Écriture protégée dans le fichier
        let mut file = log_file.lock().await;
        file.write_all(entry.as_bytes())?;
        file.flush()?;

        println!("[LOG {}] {}", addr, line);
    }

    println!("[INFO] Déconnexion de {}", addr);
    Ok(())
}
