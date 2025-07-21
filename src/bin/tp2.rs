use std::io;
use std::collections::HashMap;

// === Définition de la structure ===
#[derive(Debug, Clone)]
struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) {
        println!("💰 Compte de {} : {:.2} €", self.nom, self.solde);
    }

    fn deposer(&mut self, montant: f64) {
        if montant > 0.0 {
            self.solde += montant;
            println!("✅ +{:.2} € déposés.", montant);
        } else {
            println!("❌ Montant invalide.");
        }
    }

    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("✅ -{:.2} € retirés.", montant);
        } else {
            println!("❌ Solde insuffisant.");
        }
    }

    fn renommer(&mut self, nouveau_nom: &str) {
        println!("🔄 Compte renommé en {}.", nouveau_nom);
        self.nom = nouveau_nom.to_string();
    }

    fn fermer(self) {
        println!("🛑 Compte de {} fermé. Solde final : {:.2} €", self.nom, self.solde);
    }
}

fn main() {
    let mut comptes: HashMap<String, CompteBancaire> = HashMap::new();

    comptes.insert("Alice".to_string(), CompteBancaire { nom: "Alice".to_string(), solde: 500.0 });
    comptes.insert("Bob".to_string(), CompteBancaire { nom: "Bob".to_string(), solde: 300.0 });

    loop {
        println!("\n=== MENU BANCAIRE ===");
        println!("1. Afficher un compte");
        println!("2. Déposer de l'argent");
        println!("3. Retirer de l'argent");
        println!("4. Renommer un compte");
        println!("5. Fermer un compte");
        println!("6. Liste des comptes");
        println!("7. Quitter");
        println!("Choisissez une option :");

        let choix = lire_entier();

        match choix {
            1 => {
                if let Some(compte) = demander_compte(&comptes) {
                    compte.afficher();
                }
            }
            2 => {
                if let Some(compte) = demander_compte_mut(&mut comptes) {
                    println!("Montant à déposer :");
                    let montant = lire_decimal();
                    compte.deposer(montant);
                }
            }
            3 => {
                if let Some(compte) = demander_compte_mut(&mut comptes) {
                    println!("Montant à retirer :");
                    let montant = lire_decimal();
                    compte.retirer(montant);
                }
            }
            4 => {
                // ✅ Solution E0499 : retirer puis réinsérer
                println!("Nom du compte à renommer :");
                let ancien_nom = lire_chaine();

                if let Some(mut compte) = comptes.remove(&ancien_nom) {
                    println!("Nouveau nom du compte :");
                    let nouveau_nom = lire_chaine();
                    compte.renommer(&nouveau_nom);
                    comptes.insert(nouveau_nom, compte);
                } else {
                    println!("❌ Compte introuvable.");
                }
            }
            5 => {
                println!("Nom du compte à fermer :");
                let nom = lire_chaine();
                if let Some(compte) = comptes.remove(&nom) {
                    compte.fermer();
                } else {
                    println!("❌ Ce compte n'existe pas.");
                }
            }
            6 => {
                println!("📋 Liste des comptes :");
                for compte in comptes.values() {
                    compte.afficher();
                }
            }
            7 => {
                println!("👋 Au revoir !");
                break;
            }
            _ => println!("❗ Choix invalide."),
        }
    }
}

// === Fonctions utilitaires ===

fn lire_chaine() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn lire_entier() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lecture");
    input.trim().parse::<u32>().unwrap_or(0)
}

fn lire_decimal() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lecture");
    input.trim().parse::<f64>().unwrap_or(0.0)
}

fn demander_compte<'a>(comptes: &'a HashMap<String, CompteBancaire>) -> Option<&'a CompteBancaire> {
    println!("Nom du compte :");
    let nom = lire_chaine();
    comptes.get(&nom)
}

fn demander_compte_mut<'a>(comptes: &'a mut HashMap<String, CompteBancaire>) -> Option<&'a mut CompteBancaire> {
    println!("Nom du compte :");
    let nom = lire_chaine();
    comptes.get_mut(&nom)
}
