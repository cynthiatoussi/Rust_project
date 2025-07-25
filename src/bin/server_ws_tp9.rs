use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.expect("Erreur de bind");

    println!("🟢 Serveur WebSocket en écoute sur ws://127.0.0.1:9001");

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Erreur de connexion : {}", e);
                continue;
            }
        };

        println!("Connexion entrante de {}", addr);

        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("Erreur lors du handshake WebSocket : {}", e);
                    return;
                }
            };

            println!("[OK] WebSocket établi avec {}", addr);
            let (mut write, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(msg) => {
                        println!("📩 Message reçu : {}", msg);
                        if msg.is_text() {
                            let response = format!("Message reçu : {}", msg.into_text().unwrap());
                            write.send(response.into()).await.unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!(" [Erreur de lecture] : {}", e);
                        break;
                    }
                }
            }

            println!("🔴 Connexion fermée : {}", addr);
        });
    }
}
