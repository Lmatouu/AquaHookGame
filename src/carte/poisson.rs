extern crate rand;
use rand::Rng;

pub struct Poisson {
    pub position: (usize, usize),
    pub espece: Box<dyn Espece>,
    pub poids: f32,
    pub taille: f32,
}

pub trait Espece {
    fn emoji(&self) -> char;
    fn poids(&self) -> f32;
    fn taille(&self) -> f32;
}

pub struct PoissonTropical;
pub struct PoissonBleu;
pub struct PoissonLune;
pub struct Crabe;
pub struct Homard;
pub struct Crevette;
pub struct Calamar;
pub struct Pieuvre;

impl Espece for PoissonTropical {
    fn emoji(&self) -> char {
        '🐠'
    }
    fn poids(&self) -> f32 {
        300.0
    }
    fn taille(&self) -> f32 {
        20.0
    }
}

impl Espece for PoissonBleu {
    fn emoji(&self) -> char {
        '🐟'
    }
    fn poids(&self) -> f32 {
        200.0
    }
    fn taille(&self) -> f32 {
        15.0
    }
}

impl Espece for PoissonLune {
    fn emoji(&self) -> char {
        '🐡'
    }
    fn poids(&self) -> f32 {
        100.0
    }
    fn taille(&self) -> f32 {
        10.0
    }
}

impl Espece for Crabe {
    fn emoji(&self) -> char {
        '🦀'
    }
    fn poids(&self) -> f32 {
        50.0
    }
    fn taille(&self) -> f32 {
        5.0
    }
}

impl Espece for Homard {
    fn emoji(&self) -> char {
        '🦞'
    }
    fn poids(&self) -> f32 {
        75.0
    }
    fn taille(&self) -> f32 {
        7.0
    }
}

impl Espece for Crevette {
    fn emoji(&self) -> char {
        '🦐'
    }
    fn poids(&self) -> f32 {
        10.0
    }
    fn taille(&self) -> f32 {
        1.0
    }
}

impl Espece for Calamar {
    fn emoji(&self) -> char {
        '🦑'
    }
    fn poids(&self) -> f32 {
        100.0
    }
    fn taille(&self) -> f32 {
        10.0
    }
}

impl Espece for Pieuvre {
    fn emoji(&self) -> char {
        '🐙'
    }
    fn poids(&self) -> f32 {
        150.0
    }
    fn taille(&self) -> f32 {
        25.0
    }
}
impl Poisson {
    pub fn new(size: usize) -> Poisson {
        let mut rng = rand::thread_rng();
        let position = (rng.gen_range(0..size), rng.gen_range(0..size));
        let espece: Box<dyn Espece> = match rng.gen_range(0..8) {
            0 => Box::new(PoissonTropical),
            1 => Box::new(PoissonBleu),
            2 => Box::new(PoissonLune),
            3 => Box::new(Crabe),
            4 => Box::new(Homard),
            5 => Box::new(Crevette),
            6 => Box::new(Calamar),
            _ => Box::new(Pieuvre),
        };
        let poids = espece.poids();
        let taille = espece.taille();
        Poisson {
            position,
            poids,
            taille,
            espece,
        }
    }
}