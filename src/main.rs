use crossterm::event::{self, Event, KeyCode};
use std::io::{self, Write};
use std::sync::mpsc;



mod bateau;
mod boutique;
mod carte;

const SIZE: usize = 14;

fn main() {
    if let Err(e) = run_game() {
        eprintln!("Une erreur critique est survenue : {:?}", e);
    }
}

fn run_game() -> Result<(), Box<dyn std::error::Error>> {
    // Initialisation de la carte
    let mut map = carte::Carte::new(SIZE);

    let mut bateau = bateau::Bateau::new(String::from("Black Pearl"), ((SIZE / 2), (SIZE / 2)));

    let mut options_achetees = vec![false, false, false, false, false, false];


    let (tx, rx) = mpsc::channel();

    // Démarrer le thread qui envoie un input pour ajouter un poisson toutes les X secondes
    map.start_poisson_thread(tx);
   
    // let (input_tx, input_rx) = mpsc::channel();
    // std::thread::spawn(move || {
    //     loop {
    //         if let Ok(input) = read_input() {
    //             if input_tx.send(input).is_err() {
    //                 break;
    //             }
    //         }
    //     }
    // });

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

        // Lecture de l'entrée utilisateur
        let input = read_input()?;
        bateau.position = bateau::Bateau::move_boat(bateau.position, input, SIZE);
        
        if input == 'x' {
            println!("Au revoir !");
            break;
        }

       // Ajouter un poisson sur la carte dès que message reçu par le channel rx 
       if let Ok('p') = rx.try_recv() {
        carte::Carte::ajouter_poisson(&mut map);
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
