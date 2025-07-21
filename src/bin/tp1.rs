use std::collections::HashMap;
use std::io;

// === Définition de la structure CompteBancaire ===
struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    // Affiche le solde
    fn afficher_solde(&self) {
        println!("Le solde de {} est de {:.2} €", self.nom, self.solde);
    }

    // Effectue un retrait si le solde est suffisant
    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("Retrait de {:.2} € effectué pour {}.", montant, self.nom);
        } else {
            println!("Fonds insuffisants sur le compte de {}.", self.nom);
        }
    }
}

// === Fonction principale ===
fn main() {
    // Création de plusieurs comptes bancaires
    let mut comptes: HashMap<String, CompteBancaire> = HashMap::new();

    comptes.insert("Alice".to_string(), CompteBancaire { nom: "Alice".to_string(), solde: 500.0 });
    comptes.insert("Bob".to_string(), CompteBancaire { nom: "Bob".to_string(), solde: 350.0 });
    comptes.insert("Chloé".to_string(), CompteBancaire { nom: "Chloé".to_string(), solde: 780.0 });
    comptes.insert("David".to_string(), CompteBancaire { nom: "David".to_string(), solde: 1000.0 });

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    loop {
        println!("\n--- MENU ---");
        for (i, opt) in options.iter().enumerate() {
            println!("{} - {}", i + 1, opt);
        }

        println!("Entrez le numéro de l'option :");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim().parse::<usize>();

        match choix {
            Ok(1) => {
                // Affichage du solde
                let nom = demander_nom();
                if let Some(compte) = comptes.get(&nom) {
                    compte.afficher_solde();
                } else {
                    println!("Compte introuvable.");
                }
            }

            Ok(2) => {
                // Retrait
                let nom = demander_nom();
                if let Some(compte) = comptes.get_mut(&nom) {
                    println!("Montant à retirer :");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                    if let Ok(m) = montant.trim().parse::<f64>() {
                        compte.retirer(m);
                    } else {
                        println!("Montant invalide.");
                    }
                } else {
                    println!("Compte introuvable.");
                }
            }

            Ok(3) => {
                // Liste des comptes
                println!("--- Liste des comptes ---");
                for compte in comptes.values() {
                    println!("{} : {:.2} €", compte.nom, compte.solde);
                }
            }

            Ok(4) => {
                println!("Au revoir !");
                break;
            }

            _ => println!("Choix invalide. Réessayez."),
        }
    }
}

// === Fonction pour demander un nom d'utilisateur ===
fn demander_nom() -> String {
    println!("Entrez le nom du compte :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur de lecture");
    nom.trim().to_string()
}
