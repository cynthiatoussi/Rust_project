use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://127.0.0.1:9001").unwrap();

    println!("🔌 Connexion à {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Échec de connexion WebSocket");
    println!("[OK] Connecté au serveur WebSocket");

    let (mut write, mut read) = ws_stream.split();
    let mut input = BufReader::new(stdin()).lines();

    // Tâche pour lire les messages du serveur
    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => {
                    println!("📨 Reçu du serveur : {}", msg);
                }
                Err(e) => {
                    println!(" [Erreur] : {}", e);
                    break;
                }
            }
        }
    });

    // Lecture utilisateur en boucle
    println!("(Tapez vos messages)");
    while let Ok(Some(line)) = input.next_line().await {
        if line.trim().is_empty() {
            continue;
        }
        if let Err(e) = write.send(line.into()).await {
            println!("Erreur d'envoi : {}", e);
            break;
        }
    }
}
