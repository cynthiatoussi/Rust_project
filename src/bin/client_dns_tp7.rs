use tokio::net::UdpSocket;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?; // port local aléatoire
    let server: SocketAddr = "127.0.0.1:5300".parse().unwrap();

    let domain = "test.com";
    let message = build_dns_query(domain);

    //  Envoi de la requête
    socket.send_to(&message, &server).await?;
    println!("Requête DNS envoyée pour {}", domain);

    //  Réception de la réponse
    let mut buf = [0u8; 512];
    let (len, _) = socket.recv_from(&mut buf).await?;
    println!("Réponse reçue : {} octets", len);

    if let Some(ip) = parse_dns_response(&buf[..len]) {
        println!("Adresse IP de {} : {}", domain, ip);
    } else {
        println!("Impossible de lire l'adresse IP.");
    }

    Ok(())
}

//  Génère une requête DNS basique pour un domaine
fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut packet = Vec::new();

    // ID
    packet.push(0x12);
    packet.push(0x34);

    // Flags: standard query
    packet.push(0x01);
    packet.push(0x00);

    // QDCOUNT = 1
    packet.extend_from_slice(&[0x00, 0x01]);
    // ANCOUNT, NSCOUNT, ARCOUNT = 0
    packet.extend_from_slice(&[0x00, 0x00]); // Answer
    packet.extend_from_slice(&[0x00, 0x00]); // Authority
    packet.extend_from_slice(&[0x00, 0x00]); // Additional

    // QNAME (www.test.com -> [3]www[4]test[3]com[0])
    for label in domain.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0x00); // Terminaison

    // QTYPE = A, QCLASS = IN
    packet.extend_from_slice(&[0x00, 0x01]); // Type A
    packet.extend_from_slice(&[0x00, 0x01]); // Class IN

    packet
}

// 🔍 Extrait l'IP de la réponse DNS
fn parse_dns_response(buf: &[u8]) -> Option<String> {
    if buf.len() < 32 {
        return None;
    }

    // L’adresse IP se trouve typiquement à la fin :
    let ip_start = buf.len() - 4;
    Some(format!("{}.{}.{}.{}", buf[ip_start], buf[ip_start + 1], buf[ip_start + 2], buf[ip_start + 3]))
}
