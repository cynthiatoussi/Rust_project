// src/bin/dns_server.rs

use tokio::net::UdpSocket;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:5300").await?;
    println!("Serveur DNS en écoute sur 127.0.0.1:5300");

    let mut buf = [0u8; 512];

    loop {
        let (len, client_addr) = socket.recv_from(&mut buf).await?;
        println!("Requête reçue de {}", client_addr);

        // On parse le domaine demandé
        if let Some(domain) = extract_domain(&buf) {
            println!("Domaine demandé : {}", domain);

            // Si le domaine correspond à un domaine fictif, on répond avec une IP
            if domain == "test.com" {
                let response = build_dns_response(&buf[..len], [192, 168, 1, 42]);
                socket.send_to(&response, client_addr).await?;
                println!("Réponse envoyée");
            }
        }
    }
}

//  Extraction du nom de domaine depuis le buffer DNS
fn extract_domain(buf: &[u8]) -> Option<String> {
    let mut domain = String::new();
    let mut pos = 12; // le header DNS fait 12 octets
    while pos < buf.len() {
        let len = buf[pos] as usize;
        if len == 0 {
            break;
        }
        pos += 1;
        if pos + len > buf.len() {
            return None;
        }
        let label = std::str::from_utf8(&buf[pos..pos+len]).ok()?;
        domain.push_str(label);
        domain.push('.');
        pos += len;
    }
    if domain.ends_with('.') {
        domain.pop();
    }
    Some(domain)
}

//  Construction de la réponse DNS avec IP fixe
fn build_dns_response(request: &[u8], ip: [u8; 4]) -> Vec<u8> {
    let mut response = Vec::new();
    
    // Copier ID
    response.extend_from_slice(&request[0..2]);
    response.extend_from_slice(&[0x81, 0x80]); // Flags: standard response, no error
    response.extend_from_slice(&[0x00, 0x01]); // QDCOUNT
    response.extend_from_slice(&[0x00, 0x01]); // ANCOUNT
    response.extend_from_slice(&[0x00, 0x00]); // NSCOUNT
    response.extend_from_slice(&[0x00, 0x00]); // ARCOUNT

    // Copier la question
    let question_len = request.len() - 12;
    response.extend_from_slice(&request[12..12 + question_len]);

    // Réponse
    response.extend_from_slice(&[0xC0, 0x0C]); // Pointer vers QNAME
    response.extend_from_slice(&[0x00, 0x01]); // TYPE A
    response.extend_from_slice(&[0x00, 0x01]); // CLASS IN
    response.extend_from_slice(&[0x00, 0x00, 0x00, 0x3C]); // TTL 60s
    response.extend_from_slice(&[0x00, 0x04]); // Data length
    response.extend_from_slice(&ip); // Adresse IP

    response
}
