use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    sender: String,
    content: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7000").await?;
    println!("Serveur de messagerie en écoute sur 127.0.0.1:7000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connexion de {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => break, // déconnexion
                    Ok(n) => n,
                    Err(_) => break,
                };

                // Désérialisation du message reçu
                if let Ok(msg) = serde_json::from_slice::<Message>(&buf[..n]) {
                    println!("[{}] {} : {}", msg.timestamp, msg.sender, msg.content);
                } else {
                    println!("⚠️ Message non conforme.");
                }
            }

            println!("Client déconnecté : {}", addr);
        });
    }
}
