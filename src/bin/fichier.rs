use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use chrono::Local;

// Structure représentant un fichier
struct Fichier {
    nom: String,
}

impl Fichier {
    // Crée un nouveau fichier
    fn creer(&self) {
        let chemin = Path::new(&self.nom);
        if chemin.exists() {
            println!("[ERREUR] Le fichier '{}' existe déjà.", self.nom);
        } else {
            match OpenOptions::new().write(true).create(true).open(&self.nom) {
                Ok(_) => println!("[OK] Fichier '{}' créé à {}", self.nom, Local::now()),
                Err(e) => println!("[ERREUR] Création échouée : {}", e),
            }
        }
    }

    // Écrit du contenu dans le fichier (remplace l'ancien contenu)
    fn ecrire(&self) {
        println!("Entrez le texte à écrire dans le fichier :");
        let mut contenu = String::new();
        io::stdin().read_line(&mut contenu).unwrap();

        match fs::write(&self.nom, contenu) {
            Ok(_) => println!("[OK] Écriture réussie dans '{}'", self.nom),
            Err(e) => println!("[ERREUR] Écriture échouée : {}", e),
        }
    }

    // Lit le contenu du fichier
    fn lire(&self) {
        match fs::read_to_string(&self.nom) {
            Ok(data) => {
                println!("[OK] Contenu de '{}':\n{}", self.nom, data);
            }
            Err(e) => println!("[ERREUR] Lecture échouée : {}", e),
        }
    }

    // Modifie (ajoute à la fin)
    fn modifier(&self) {
        println!("Entrez le texte à ajouter :");
        let mut ajout = String::new();
        io::stdin().read_line(&mut ajout).unwrap();

        match OpenOptions::new().append(true).open(&self.nom) {
            Ok(mut fichier) => {
                if let Err(e) = writeln!(fichier, "\n{}", ajout.trim()) {
                    println!("[ERREUR] Erreur d'écriture : {}", e);
                } else {
                    println!("[OK] Modification réussie à {}", Local::now());
                }
            }
            Err(e) => println!("[ERREUR] Ouverture échouée : {}", e),
        }
    }

    // Supprime définitivement le fichier
    fn supprimer(&self) {
        match fs::remove_file(&self.nom) {
            Ok(_) => println!("[OK] Fichier '{}' supprimé à {}", self.nom, Local::now()),
            Err(e) => println!("[ERREUR] Suppression échouée : {}", e),
        }
    }
}

// Fonction principale
fn main() {
    println!("Bienvenue dans le Gestionnaire de fichiers (Rust)");

    loop {
        println!("\n--- MENU ---");
        println!("1. Créer un fichier");
        println!("2. Écrire dans un fichier");
        println!("3. Lire un fichier");
        println!("4. Modifier un fichier");
        println!("5. Supprimer un fichier");
        println!("0. Quitter");
        print!("Choix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        let choix = choix.trim();

        match choix {
            "1" | "2" | "3" | "4" | "5" => {
                print!("Nom du fichier : ");
                io::stdout().flush().unwrap();
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let fichier = Fichier { nom: nom.trim().to_string() };

                match choix {
                    "1" => fichier.creer(),
                    "2" => fichier.ecrire(),
                    "3" => fichier.lire(),
                    "4" => fichier.modifier(),
                    "5" => fichier.supprimer(),
                    _ => (),
                }
            }
            "0" => {
                println!("Fin du programme. À bientôt.");
                break;
            }
            _ => {
                println!("[ERREUR] Choix invalide.");
            }
        }
    }
}
