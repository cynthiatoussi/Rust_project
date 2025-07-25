use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{stdin, stdout, Write};

#[derive(Serialize)]
struct Message {
    sender: String,
    content: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7000").await?;
    println!("[OK] Connecté au serveur de messagerie");

    // Prompt pour pseudo
    print!("Entrez votre pseudo : ");
    stdout().flush().unwrap();
    let mut pseudo = String::new();
    stdin().read_line(&mut pseudo).unwrap();
    let pseudo = pseudo.trim().to_string();

    println!("(Tapez vos messages et appuyez sur Entrée)");

    loop {
        let mut input = String::new();

        // Lecture synchrone ligne par ligne (plus robuste)
        print!("> ");
        stdout().flush().unwrap();
        if stdin().read_line(&mut input).is_err() {
            println!("Erreur de lecture. Arrêt.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        let msg = Message {
            sender: pseudo.clone(),
            content: trimmed.to_string(),
            timestamp: current_timestamp(),
        };

        let serialized = serde_json::to_string(&msg).unwrap();
        if let Err(e) = stream.write_all(serialized.as_bytes()).await {
            println!("Erreur lors de l'envoi : {}", e);
            break;
        }
    }

    Ok(())
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
