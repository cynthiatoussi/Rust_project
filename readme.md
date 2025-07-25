## Synthèse de la séance 1: Programmation en Rust

### 🔹 1. Les bases du langage Rust

* **Types de données** : `i32`, `u32`, `f32`, `String`, etc.
* **Conventions** : noms en `snake_case`, pas de tirets ni de chiffres au début.
* **Variables** : `let`, typage explicite (`let age: u32 = 30`) ou implicite.

---

### 🔹 2. Les fonctions

* Déclaration avec `fn`
* Paramètres typés, valeurs de retour (`-> i32`)
* Appels depuis `main()`
* Exemples :

  * `fn addition(n1: i32, n2: i32) -> i32 { ... }`
  * `fn say_hello(nom: &str)`

---

### 🔹 3. Structures (`struct`) et implémentation (`impl`)

* Définition de types personnalisés comme `Salarie`, `Personne`, `CompteBancaire`
* Implémentation de méthodes avec `impl`
* Types d’accès :

  * `&self` → lecture
  * `&mut self` → modification
  * `self` → consommation (l’objet n’est plus utilisable ensuite)

---

### 🔹 4. Structure principale : ex: `CompteBancaire`

* Attributs : `nom` et `solde`
* Méthodes :

  * `afficher()`
  * `deposer(montant)`
  * `retirer(montant)`
  * `fermer()` → consomme l’objet
* **Points bonus proposés** :

  * Interdire dépôt négatif
  * Renommer un compte (nouvelle instance)
  * Gérer plusieurs comptes via `Vec<CompteBancaire>`

---

### 🔹 5. Les conditions et boucles

* `if` pour tester parité, bornes, etc.
* Boucles :

  * `for i in 1..=10`
  * `loop { ... break; }`
  * `while compteur < 5`
* **Itérations avec index** via `.enumerate()`
* Utilisation de `.iter()` pour itérer sans consommer les éléments

---

### 🔹 6. Collections

* Tableaux statiques `[i32; 4]`
* Vecteurs dynamiques `vec![...]`
* Exemples :

  * `for &elt in &tab` : itération sans transfert de propriété
  * Affichage de menus dynamiques avec `Vec` et `enumerate()`

---

### 🔹 7. Le `match`

* Similaire à `switch` en C/Java, mais plus puissant
* Exemple :

```rust
match nombre {
    1 => println!("Un"),
    2 => println!("Deux"),
    _ => println!("Autre")
}
```


----------------------------------------------------------------------------------------------------------

## Synthèse de la séance 2: Programmation en Rust

Les notions suivantes ont été abordées :

**Structures (`struct`)** : encapsulation des propriétés d’un fichier dans une structure `Fichier`.
**Implémentation de méthodes (`impl`)** : toutes les opérations (créer, lire, écrire...) sont définies dans un bloc `impl` lié à la structure.
**Gestion de l'ownership et du borrowing** : passage de `&self` dans les méthodes pour respecter le modèle d’ownership de Rust.
**Utilisation de `match`, `loop`, `while`** : menu principal interactif basé sur des structures de contrôle.
**Gestion des fichiers** avec `std::fs` et `OpenOptions`.
**Utilisation de la bibliothèque `chrono`** : ajout de la date/heure aux opérations pour le suivi.
**Manipulation de chemins de fichiers** : via `std::path::Path` pour vérifier l'existence.
**Gestion des erreurs (`Result`, `match`)** : traitement propre des erreurs lors des lectures/écritures.

**Fonctions asynchrones (async fn) et await** : exécution non bloquante de tâches.
**Macro #[tokio::main]** : transforme main() en fonction asynchrone avec le runtime Tokio.
**Gestion des connexions TCP** : TcpListener::bind, TcpStream, accept().await.
**Lecture asynchrone ligne par ligne** : via tokio::io::BufReader et .lines().await.
**Partage de ressources entre tâches** : Arc<T> pour le comptage de références partagé.
**Accès concurrent sécurisé** : tokio::sync::Mutex<T> pour protéger le fichier partagé.
**Création de tâches concurrentes** : tokio::spawn(async move { ... }).
**Formatage d’horodatage** : chrono::Utc::now().to_rfc3339().

-------------------------------------------------------------------------------------------------------------------------------

## Synthèse de la séance 3: Programmation en Rust


## TP 7 : Serveur et Client DNS (UDP)

### Objectif :

Implémentation d'un client DNS qui envoie des requêtes pour résoudre un nom de domaine, et un serveur DNS simplifié qui répond avec une adresse IP codée en dur.

### Fonctionnalités :

* Envoi/réception de paquets DNS via UDP avec `tokio::net::UdpSocket`
* Extraction du nom de domaine à partir du buffer binaire (RFC 1035)
* Construction manuelle d'une réponse DNS binaire avec entête et réponse IP

### Implémentation du code :

on lance `server_dns_tp7.rs` avec la commande `cargo run --bin server_dns_tp7`

![alt text](image-1.png)

puis `client_dns_tp7.rs` avec la commande `cargo run --bin client_dns_tp7`

![alt text](image.png)

---

##  TP 8 : Protocole personnalisé de messagerie (TCP)

### Objectif :

j'ai choisi de créer un protocole de messagerie au-dessus de TCP, avec un format JSON personnalisé contenant un expéditeur, un contenu et un horodatage.

### Fonctionnalités :

* Communication via `tokio::net::TcpStream` et `TcpListener`
* Structure `Message` sérialisée avec `serde_json`
* Gestion de plusieurs clients en tâches parallèles (serveur non bloquant)
* Client interactif en ligne de commande avec pseudo

### Implémentation du code :

on lance `server_chat_tp8.rs` avec la commande `cargo run --bin server_chat_tp8`

![alt text](image-3.png)

puis `client_chat_tp8.rs` avec la commande `cargo run --bin client_chat_tp8`

![alt text](image-2.png)

---

##  TP 9 : Messagerie WebSocket (Tokio + Tungstenite)

### Objectif :

Utilisation d'un serveur et d'un client WebSocket pour des communications full-duplex en temps réel, inspirées des chats modernes.

### Fonctionnalités :

* Serveur WebSocket avec `tokio-tungstenite`
* Client WebSocket qui lit les entrées utilisateur et reçoit les messages du serveur
* Splitting des flux (`read` / `write`) pour paralléliser l'envoi et la réception

### Implementation du code :

on lance `server_ws_tp9.rs` avec la commande `cargo run --bin server_ws_tp9`

![alt text](image-5.png)

puis `client_ws_tp9.rs` avec la commande `cargo run --bin client_ws_tp9`

![alt text](image-4.png)

---
