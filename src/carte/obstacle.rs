extern crate rand;
use rand::Rng;

pub struct Cyclone;
pub struct Requin;
pub struct Pirate;

pub trait Mechant {
    fn emoji(&self) -> char;
    fn attaque(&self) -> i32;
    fn name(&self) -> String;}

impl Mechant for Cyclone {
    fn emoji(&self) -> char {
        '🌀'
    }
    fn attaque(&self) -> i32 {
        25
    }
    fn name(&self) -> String {
        String::from("Cyclone")
    }
}

impl Mechant for Requin {
    fn emoji(&self) -> char {
        '🦈'
    }
    fn attaque(&self) -> i32 {
        50
    }
    fn name(&self) -> String {
        String::from("Requin")
    }
}

impl Mechant for Pirate {
    fn emoji(&self) -> char {
        '💀'
    }
    fn attaque(&self) -> i32 {
        50
    }
    fn name(&self) -> String {
        String::from("Pirate")
    }
}

pub struct Obstacle {
    pub position: (usize, usize),
    pub espece: Box<dyn Mechant>,
    pub attaque: i32,
    pub name: String,
}

impl Obstacle {
    pub fn new(size: usize) -> Obstacle {
        let mut rng = rand::thread_rng();
        let position = (rng.gen_range(0..size), rng.gen_range(0..size));

        let espece: Box<dyn Mechant> = match rng.gen_range(0..3) {
            0 => Box::new(Cyclone),
            1 => Box::new(Requin),
            _ => Box::new(Pirate),
        };

        let attaque = espece.attaque();
        let name = espece.name();

        Obstacle {
            position,
            espece,
            attaque,
            name,
        }
    }
}

