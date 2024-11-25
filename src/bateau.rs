use std::sync::Mutex;
use std::fmt;

pub struct Bateau {
    pub nom: String,
    pub position: (usize, usize),
    pub pv: i32,
    pub pv_max: i32,
    pub cale: i32,
    pub cale_max: i32,
    pub tresor: i32,
    pub emoji: char,
}

impl Bateau {
    pub fn new(nom: String, position: (usize, usize)) -> Bateau {
        Bateau {
            nom,
            position,
            pv: 50,
            pv_max: 100,
            cale: 0,
            cale_max: 5,
            tresor: 1500,
            emoji: '🛶',
        }
    }

    pub fn status(&self) {
        println!(
            "{} {} a {}/{} points de vie, {}/{} poissons dans la cale et {} 🪙",
            self.nom, self.emoji, self.pv, self.pv_max, self.cale, self.cale_max, self.tresor
        );
    }

    pub fn move_boat(position: (usize, usize), input: char, size: usize) -> (usize, usize) {
        let (mut x, mut y) = position;
        match input {
            'z' if x > 0 => x -= 1,        // Haut
            's' if x < size - 1 => x += 1, // Bas
            'q' if y > 0 => y -= 1,        // Gauche
            'd' if y < size - 1 => y += 1, // Droite
            _ => {}                        // Entrée invalide ou bord de la carte
        }
        (x, y)
    }

    pub fn add_poisson_cale(&mut self, quantite: i32) -> Result<(), String> {
        // Gestion de l'accès concurrent avec Mutex et gestion des erreurs de lock
        let cale = Mutex::new(self.cale);
        let mut cale_guard = match cale.lock() {
            Ok(guard) => guard,
            Err(_) => return Err("Erreur lors du verrouillage du Mutex".to_string()),
        };

        *cale_guard += quantite;

        // Vérification que la cale ne dépasse pas sa capacité
        if *cale_guard > self.cale_max {
            return Err(format!("La cale dépasse la capacité maximale de {} poissons.", self.cale_max));
        }

        self.cale = *cale_guard;
        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.cale >= self.cale_max
    }

    pub fn receive_damage(&mut self, damage: i32) -> Result<(), String> {
        // Vérification pour éviter que les points de vie tombent en dessous de 0
        if damage < 0 {
            return Err("La valeur des dégâts ne peut pas être négative.".to_string());
        }

        self.pv -= damage;

        // Assurer que les points de vie ne soient pas négatifs
        if self.pv < 0 {
            self.pv = 0;
        }
        Ok(())
    }

    pub fn heal(&mut self, heal_points: i32) -> Result<(), String> {
        // Vérification pour éviter que les points de vie dépassent le maximum
        if heal_points < 0 {
            return Err("Les points de soin ne peuvent pas être négatifs.".to_string());
        }

        self.pv += heal_points;

        // S'assurer que les points de vie ne dépassent pas le maximum
        if self.pv > self.pv_max {
            self.pv = self.pv_max;
        }
        Ok(())
    }
}

// Implémentation de la fonction fmt::Debug pour afficher un bateau de manière lisible
impl fmt::Debug for Bateau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} à la position {:?} avec {} points de vie et {} poissons dans la cale",
            self.nom, self.emoji, self.position, self.pv, self.cale
        )
    }
}
