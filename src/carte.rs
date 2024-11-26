mod ile;
mod obstacle;
pub mod poisson;
use rand::Rng;


use crate::bateau::Bateau;
use ile::Ile;
use obstacle::Obstacle;
use poisson::Poisson;

const NUM_ILES: usize = 3;
const NUM_POISSONS: usize = 8;
const NUM_OBSTACLES: usize = 3;
const NUM_MAX_POISSONS: usize = 20;
const NUM_MAX_OBSTACLES: usize = 10;

pub struct Carte {
    pub taille: usize,
    pub map: Vec<Vec<char>>,
    pub iles: Vec<Ile>,
    pub poissons: Vec<Poisson>,
    pub obstacles: Vec<Obstacle>,
}

impl Carte {
    pub fn new(taille: usize) -> Carte {
        let mut iles = vec![];
        for _ in 0..NUM_ILES {
            iles.push(Ile::new(taille));
        }

        let mut poissons = vec![];
        for _ in 0..NUM_POISSONS {
            poissons.push(Poisson::new(taille));
        }

        let mut obstacles = vec![];
        for _ in 0..NUM_OBSTACLES {
            obstacles.push(Obstacle::new(taille));
        }

        reposition_if_needed(&mut iles, &mut poissons, &mut obstacles, taille);

        Carte {
            taille: taille,
            map: vec![vec!['🟦'; taille]; taille],
            iles: iles,
            poissons: poissons,
            obstacles: obstacles,
        }
    }

    pub fn print_map(&self) {
        for row in &self.map {
            for cell in row {
                print!("{}", cell);
            }
            print!("\n");
        }
    }
    pub fn update_map(&mut self, bateau: &mut Bateau) {
        self.bateau_sur_poisson(bateau);

        self.bateau_sur_obstacles(bateau);

        for row in self.map.iter_mut() {
            for cell in row.iter_mut() {
                *cell = '🟦';
            }
        }

        // Ajoute les îles et poissons sur la carte
        for ile in &self.iles {
            let (x, y) = ile.position;
            self.map[x][y] = '🟧';
        }

        for poisson in &self.poissons {
            let (x, y) = poisson.position;
            self.map[x][y] = poisson.espece.emoji();
        }

        for obstacle in &self.obstacles {
            let (x, y) = obstacle.position;
            self.map[x][y] = obstacle.espece.emoji();
        }

        // Ajoute le bateau sur la carte
        let (x, y) = bateau.position;
        self.map[x][y] = bateau.emoji;

        if bateau.is_alive() == false {
            self.map[x][y] = '💥';
            let directions_explosions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
            for (dx, dy) in directions_explosions.iter() {
                let new_flamme_x = x as isize + dx;
                let new_flamme_y = y as isize + dy;
                if new_flamme_x >= 0 && new_flamme_x < self.taille as isize && new_flamme_y >= 0 && new_flamme_y < self.taille as isize {
                    self.map[new_flamme_x as usize][new_flamme_y as usize] = '🔥';
                }
            }
        }

    }

    pub fn bateau_sur_ile(&self, boat_position: (usize, usize)) -> bool {
        for ile in &self.iles {
            if boat_position == ile.position {
                println!("Vous êtes sur une île.");
                return true;
            }
        }
        return false;
    }

    pub fn bateau_sur_poisson(&mut self, bateau: &mut Bateau) {
        for poisson in &self.poissons {
            if bateau.position == poisson.position {
                if !bateau.is_full() {
                    if let Some(index) = self
                        .poissons
                        .iter()
                        .position(|p| p.position == poisson.position)
                    {
                        let poisson = self.poissons.remove(index);
                        if let Err(e) = bateau.add_poisson_cale(poisson) {
                            eprintln!("Erreur : {}", e);
                        }
                    }
                } else {
                    println!("La cale est pleine !");
                }
                break;
            }
        }
    }

    pub fn bateau_sur_obstacles(&mut self, bateau: &mut Bateau) {
        for obstacle in &self.obstacles {
            if bateau.position == obstacle.position {
                match bateau.receive_damage(obstacle.attaque) {
                    Ok(()) => {
                        println!(
                            "Vous avez heurté un {} et perdu {} points de vie. \n",
                            obstacle.name, obstacle.attaque
                        );
                    }
                    Err(e) => {
                        eprintln!("Erreur lors de la réception des dégâts : {}", e);
                    }
                }

                if obstacle.name == "Pirate" {
                    match bateau.remove_tresor(obstacle.vol_piece) {
                        Ok(()) => {
                            print!("Vous vous êtes fait voler {} pièces.\n",
                            obstacle.vol_piece
                        );
                    }
                    Err(e) => {
                        eprintln!("Erreur lors du retrait du trésor : {}", e);
                    }
                }
            }
                break;
            }
        }
    }

    pub fn ajouter_poisson(&mut self) {
        if self.poissons.len() < NUM_MAX_POISSONS {
            self.poissons.push(Poisson::new(self.taille));
            reposition_if_needed(&mut self.iles, &mut self.poissons, &mut self.obstacles, self.taille);
        } 
    }

    pub fn ajouter_obstacle(&mut self) {
        if self.poissons.len() < NUM_MAX_OBSTACLES {
            self.obstacles.push(Obstacle::new(self.taille));
            reposition_if_needed(&mut self.iles, &mut self.poissons, &mut self.obstacles, self.taille);
        } 
    }

    pub fn start_poisson_thread(&self, tx: std::sync::mpsc::Sender<char>) {
        std::thread::spawn(move || {
            loop {
                let delay = rand::thread_rng().gen_range(1..=3);
                std::thread::sleep(std::time::Duration::from_secs(delay));
                if tx.send('p').is_err() {
                    break;
                }
            }
        });
    }

    pub fn start_obsctacle_thread(&self, tx: std::sync::mpsc::Sender<char>) {
        std::thread::spawn(move || {
            loop {
                let delay = rand::thread_rng().gen_range(1..=5);
                std::thread::sleep(std::time::Duration::from_secs(delay));
                if tx.send('o').is_err() {
                    break;
                }
            }
        });
    }

    pub fn start_deplacement_thread(&self, tx: std::sync::mpsc::Sender<char>) {
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(1500));
                if tx.send('d').is_err() {
                    break;
                }
            }
        });
    }
    
    pub fn deplacer_poissons_et_obstacles(&mut self) {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut rng = rand::thread_rng();

        for poisson in &mut self.poissons {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let new_x = (poisson.position.0 as isize + dx).clamp(0, self.taille as isize - 1) as usize;
            let new_y = (poisson.position.1 as isize + dy).clamp(0, self.taille as isize - 1) as usize;
            poisson.position = (new_x, new_y);
        }

        for obstacle in &mut self.obstacles {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let new_x = (obstacle.position.0 as isize + dx).clamp(0, self.taille as isize - 1) as usize;
            let new_y = (obstacle.position.1 as isize + dy).clamp(0, self.taille as isize - 1) as usize;
            obstacle.position = (new_x, new_y);
        }

        reposition_if_needed(&mut self.iles, &mut self.poissons, &mut self.obstacles, self.taille);
    }


}

fn reposition_if_needed(
    iles: &mut Vec<Ile>,
    poissons: &mut Vec<Poisson>,
    obstacles: &mut Vec<Obstacle>,
    taille: usize,
) {
    let mut positions = std::collections::HashSet::new();

    for ile in iles.iter_mut() {
        while !positions.insert(ile.position) {
            ile.position = Ile::new(taille).position;
        }
    }

    for poisson in poissons.iter_mut() {
        while !positions.insert(poisson.position) {
            poisson.position = Poisson::new(taille).position;
        }
    }

    for obstacle in obstacles.iter_mut() {
        while !positions.insert(obstacle.position) {
            obstacle.position = Obstacle::new(taille).position;
        }
    }
}
