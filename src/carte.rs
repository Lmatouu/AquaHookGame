mod ile;
mod obstacle;
mod poisson;

use crate::bateau::Bateau;
use ile::Ile;
use obstacle::Obstacle;
use poisson::Poisson;

const NUM_ILES: usize = 3;
const NUM_POISSONS: usize = 8;
const NUM_OBSTACLES: usize = 3;

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
                print!("{} ", cell);
            }
            println!();
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
                        self.poissons.remove(index);
                    }
                    bateau.add_poisson_cale(1);
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
                bateau.receive_damage(obstacle.attaque);
                println!(
                    "Vous avez heurté un {} et perdu {} points de vie.",
                    obstacle.name, obstacle.attaque
                );
                break;
            }
        }
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
