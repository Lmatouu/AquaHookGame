extern crate rand;
use rand::Rng;

pub struct Cyclone;
pub struct Requin;
pub struct Pirate;

pub trait Mechant {
    fn emoji(&self) -> char;
    fn attaque(&self) -> i32;
    fn name(&self) -> String;
    fn vol_piece(&self) -> i32;}

impl Mechant for Cyclone {
    fn emoji(&self) -> char {
        '🌀'
    }
    fn attaque(&self) -> i32 {
        50
    }
    fn name(&self) -> String {
        String::from("Cyclone")
    }
    fn vol_piece(&self) -> i32 {
        0
    }
}

impl Mechant for Requin {
    fn emoji(&self) -> char {
        '🦈'
    }
    fn attaque(&self) -> i32 {
        10
    }
    fn name(&self) -> String {
        String::from("Requin")
    }
    fn vol_piece(&self) -> i32 {
        0
    }
}

impl Mechant for Pirate {
    fn emoji(&self) -> char {
        '💀'
    }
    fn attaque(&self) -> i32 {
        20
    }
    fn name(&self) -> String {
        String::from("Pirate")
    }
    fn vol_piece(&self) -> i32 {
        50
    }
}

pub struct Obstacle {
    pub position: (usize, usize),
    pub espece: Box<dyn Mechant>,
    pub attaque: i32,
    pub name: String,
    pub vol_piece: i32,
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
        let vol_piece = espece.vol_piece();

        Obstacle {
            position,
            espece,
            attaque,
            name,
            vol_piece,
        }
    }
}

