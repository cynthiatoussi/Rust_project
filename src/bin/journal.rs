// Importation des fonctions et types standards nécessaires à la gestion de fichiers
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::sync::Arc;

// Bibliothèque chrono pour obtenir la date et l'heure UTC
use chrono::Utc;

// Importation des modules asynchrones de lecture, réseau, synchronisation
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

/// Chemin constant vers le fichier de log
const LOG_FILE_PATH: &str = "logs/server.log";

/// Fonction principale asynchrone (lancement du serveur TCP)
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //  Crée le dossier "logs" s’il n’existe pas déjà (équivalent à mkdir -p)
    create_dir_all("logs")?;

    //  Ouvre le fichier "logs/server.log" en mode ajout
    // S’il n'existe pas, il est créé
    let file = OpenOptions::new()
        .create(true)   // crée le fichier si besoin
        .append(true)   // ajoute à la fin sans effacer
        .open(LOG_FILE_PATH)?;

    //  Partage sécurisé du fichier entre tâches avec Arc<Mutex<>>
    // Arc = compteur de références atomique (multi-tâches)
    // Mutex = protection d'accès concurrent
    let shared_file = Arc::new(Mutex::new(file));

    //  Création du serveur TCP qui écoute sur 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("[OK] Serveur lancé sur 127.0.0.1:8080");

    //  Boucle infinie : on attend des connexions entrantes
    loop {
        // Accepte une nouvelle connexion (bloque jusqu'à ce qu’un client arrive)
        let (stream, addr) = listener.accept().await?;
        println!("[INFO] Nouvelle connexion : {}", addr);

        // Clone du pointeur vers le fichier partagé pour cette nouvelle tâche
        let file_clone = Arc::clone(&shared_file);

        //  Lancement d'une nouvelle tâche pour gérer ce client
        // Elle tourne indépendamment des autres clients
        tokio::spawn(async move {
            // Appelle la fonction handle_client
            if let Err(e) = handle_client(stream, addr, file_clone).await {
                eprintln!("[ERREUR] Connexion {} : {}", addr, e);
            }
        });
    }
}


/// Gère un client connecté : lit ses messages et les enregistre dans le log
async fn handle_client(
    stream: TcpStream,                              // le canal de communication client
    addr: std::net::SocketAddr,                     // adresse IP/port du client
    log_file: Arc<Mutex<std::fs::File>>,            // fichier partagé (verrouillé)
) -> std::io::Result<()> {
    //  On enveloppe le flux dans un BufReader pour lire ligne par ligne
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();  // stream ligne par ligne (asynchrone)

    //  Tant que le client envoie des lignes, on les traite une par une
    while let Some(line) = lines.next_line().await? {
        // Horodatage du message reçu
        let timestamp = Utc::now().to_rfc3339();
        let log_line = format!("[{}]  {}\n", timestamp, line);

        {
            //  Bloc où on verrouille l'accès au fichier pour écrire (verrou libéré à la fin du bloc)
            let mut file = log_file.lock().await;
            file.write_all(log_line.as_bytes())?;  // écriture du message
            file.flush()?;                         // forcer l'écriture immédiate
        }

        // Affichage console à chaque message reçu
        println!("[LOG {}] {}", addr, line);
    }

    // Une fois la boucle terminée, le client a fermé la connexion
    println!("[INFO] Déconnexion : {}", addr);
    Ok(())
}
