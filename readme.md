## Synthèse de la séance : Programmation en Rust

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

### 🔹 4. Structure principale : `CompteBancaire`

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

---

### Compétences acquises:

* Créer et manipuler des variables typées
* Écrire et appeler des fonctions
* Définir et utiliser des structures (`struct`)
* Implémenter des méthodes avec `impl`
* Maîtriser les notions de propriété, références et mutabilité
* Manipuler des boucles, tableaux et vecteurs
* Écrire un menu interactif simple
* Utiliser le `match` pour des cas conditionnels

---

### Conclusion

Cette séance m'a permis d’acquérir une base solide en Rust, notamment :

- Approche orientée objet avec `struct` + `impl`
- Utilisation sécurisée de la mémoire (propriété, emprunt)
- Manipulation de collections et contrôle du flux

