extern crate rand;
use rand::Rng;

pub struct Ile {
    pub position: (usize, usize),
}

impl Ile {
    pub fn new(size: usize) -> Ile {
        let mut rng = rand::thread_rng();
        Ile {
            position: (rng.gen_range(0..size), rng.gen_range(0..size)),
        }
    }
}
