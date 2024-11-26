use crossterm::event::{self, Event, KeyCode};
use std::io::{self, Write};
use std::sync::mpsc;

use std::thread;

mod audio;
mod image;
mod bateau;
mod boutique;
mod carte;

const SIZE: usize = 20;

fn main() {
    thread::spawn(|| {
        let _ = audio::run_music("/home/.../musique/AquaHookMusic.mp3"); // Remplacez par le chemin de votre fichier audio
    });

    if let Err(e) = run_game() {
        eprintln!("Une erreur critique est survenue : {:?}", e);
    }
}

fn run_game() -> Result<(), Box<dyn std::error::Error>> {
    image::afficher_ecran_chargement()?;
    
    clear_terminal()?;
    // Initialisation de la carte
    let mut map = carte::Carte::new(SIZE);

    let mut bateau = bateau::Bateau::new(String::from("Black Pearl"), ((SIZE / 2), (SIZE / 2)));

    let mut options_achetees = vec![false, false, false, false, false, false];


    let (tx_poisson, rx_poisson) = mpsc::channel();
    // Démarrer le thread qui envoie un input pour ajouter un poisson toutes les X secondes
    map.start_poisson_thread(tx_poisson);

    let (tx_obstacle, rx_obstacle) = mpsc::channel();
    // Démarrer le thread qui envoie un input pour ajouter un obstacle toutes les X secondes
    map.start_obsctacle_thread(tx_obstacle);

    let (tx_deplacement, rx_deplacement): (mpsc::Sender<char>, mpsc::Receiver<char>) = mpsc::channel();
    // Démarrer le thread qui envoie un input pour déplacer les poissons et les obstacles toutes les secondes
    map.start_deplacement_thread(tx_deplacement);



    loop {
        // Vérifie si le bateau est sur une île
        let mut bateau_sur_ile = carte::Carte::bateau_sur_ile(&map, bateau.position);

        while bateau_sur_ile {
            clear_terminal()?;
            bateau.status();
            boutique::afficher_boutique(&options_achetees);

            let input = read_input()?;

            // Si l'utilisateur appuie sur une touche de déplacement (z, q, s, d), déplacer le bateau
            if ['z', 'q', 's', 'd'].contains(&input) {
                bateau.position = bateau::Bateau::move_boat(bateau.position, input, SIZE);

                // Vérifier à nouveau si le bateau est sur une île après le déplacement
                bateau_sur_ile = carte::Carte::bateau_sur_ile(&map, bateau.position);
            } else {
                // Si l'utilisateur choisit une option dans la boutique
                boutique::handle_boutique_input(&mut bateau, input, &mut options_achetees);
            }
        }

        clear_terminal()?;

        carte::Carte::update_map(&mut map, &mut bateau);
        carte::Carte::print_map(&map);
        println!("Déplacez le bateau (z: haut, q: gauche, s: bas, d: droite, x: quitter) :");
        bateau.status();

        // Si le bateau n'a plus de points de vie, le jeu est terminé
        if bateau.is_alive() == false{
            println!("Vous avez perdu !");
            break;
        }

        // Lecture de l'entrée utilisateur
        let input = read_input()?;
        bateau.position = bateau::Bateau::move_boat(bateau.position, input, SIZE);

         // Si l'utilisateur appuie sur 'x', quitter le jeu
        if input == 'x' {
            println!("Au revoir !");
            break;
        }

        // Ajouter un poisson sur la carte dès que message reçu par le channel rx 

        if let Ok('p') = rx_poisson.try_recv() {
            carte::Carte::ajouter_poisson(&mut map);
        }

        // Ajouter un obstacle sur la carte dès que message reçu par le channel rx 
       if let Ok('o') = rx_obstacle.try_recv() {
            carte::Carte::ajouter_obstacle(&mut map);
        }

        // Envoyer l'input de déplacement à la carte
        if let Ok('d') = rx_deplacement.try_recv() {
            carte::Carte::deplacer_poissons_et_obstacles(&mut map);
        }
                
    }

    Ok(())
}

// Effacer le terminal
fn clear_terminal() -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// Lire l'entrée utilisateur
fn read_input() -> Result<char, Box<dyn std::error::Error>> {
    enable_raw_mode_safe()?;

    let result = loop {
        if let Err(e) = event::poll(std::time::Duration::from_millis(100)) {
            eprintln!("Erreur lors du poll : {:?}", e);
            break 'x';
        }

        match event::read() {
            Ok(Event::Key(key_event)) if matches!(key_event.code, KeyCode::Char(_)) => {
                if let KeyCode::Char(c) = key_event.code {
                    break c;
                }
            }
            Ok(Event::Key(_)) | Ok(Event::FocusGained) | Ok(Event::FocusLost) 
            | Ok(Event::Mouse(_)) | Ok(Event::Paste(_)) | Ok(Event::Resize(_, _)) => {
                continue;
            }
            Err(e) => {
                eprintln!("Erreur de lecture d'événement : {:?}", e);
                break 'x';
            }
        }
    };

    disable_raw_mode_safe()?;

    Ok(result)
}

// Activer le mode "raw"
fn enable_raw_mode_safe() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// Désactiver le mode "raw"
fn disable_raw_mode_safe() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::disable_raw_mode().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}