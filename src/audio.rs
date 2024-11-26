use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

pub fn run_music(chemin: &str) -> Result<(), Box<dyn Error>> {
    let (_stream, handle) = OutputStream::try_default()?;

    let fichier = File::open(chemin)?;
    let source = Decoder::new(BufReader::new(fichier))?;

    let sink = Sink::try_new(&handle)?;

    // Mettre en boucle la musique
    sink.append(source);

    // Garder le programme en attente pour laisser jouer la musique
    sink.sleep_until_end();

    Ok(())
}