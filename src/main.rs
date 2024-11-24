use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};

mod bateau;
mod boutique;
mod carte;

const SIZE: usize = 12;

fn main() {
    // Initialisation de la carte
    let mut map = carte::Carte::new(SIZE);

    let mut bateau = bateau::Bateau::new(String::from("Black Pearl"), ((SIZE / 2), (SIZE / 2)));

    loop {

        // Vérifie si le bateau est sur une île
        let mut bateau_sur_ile = carte::Carte::bateau_sur_ile(&map, bateau.position);

        while bateau_sur_ile {
            clear_terminal();
            bateau.status();
            boutique::afficher_boutique();
            let input = read_input();

            // Si l'utilisateur appuie sur une touche de déplacement (z, q, s, d), déplacer le bateau
            if ['z', 'q', 's', 'd'].contains(&input) {
                // Déplacer le bateau selon la touche entrée
                bateau.position = bateau::Bateau::move_boat(bateau.position, input, SIZE);

                // Vérifier à nouveau si le bateau est sur une île après le déplacement
                bateau_sur_ile = carte::Carte::bateau_sur_ile(&map, bateau.position);
            } else {
                // Si l'utilisateur choisit une option dans la boutique
                boutique::handle_boutique_input(&mut bateau, input);
            }
        }

        clear_terminal();

        carte::Carte::update_map(&mut map, &mut bateau);
        carte::Carte::print_map(&map);
        println!("Déplacez le bateau (z: haut, q: gauche, s: bas, d: droite, x: quitter) :");
        bateau.status();

        // Lecture de l'entrée utilisateur
        let input = read_input();

        // Déplacer le bateau ou quitter
        if input == 'x' {
            println!("Au revoir !");
            break;
        }
        bateau.position = bateau::Bateau::move_boat(bateau.position, input, SIZE);
    }
}

// Effacer le terminal
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

// Lit l'entrée utilisateur
fn read_input() -> char {
    enable_raw_mode().unwrap();
    let result = loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(c) => break c,
                    _ => {}
                }
            }
        }
    };
    disable_raw_mode().unwrap();
    result
}
