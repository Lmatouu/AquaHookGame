use std::sync::Mutex;

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
            nom: nom,
            position: position,
            pv: 50,
            pv_max:100,
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

    pub fn add_poisson_cale(&mut self, quantite: i32) {

        let cale = Mutex::new(self.cale);
        {
            let mut cale_guard = cale.lock().unwrap();
            *cale_guard += quantite;
        }
        self.cale = *cale.lock().unwrap();

    }

    pub fn is_full(&self) -> bool {
        self.cale >= self.cale_max as i32
    }

    pub fn receive_damage(&mut self, damage: i32) {
        self.pv -= damage;
    }
}
