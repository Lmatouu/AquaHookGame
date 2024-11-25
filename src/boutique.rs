use crate::bateau::Bateau;

#[derive(Debug, PartialEq)]
pub enum OptionBoutique {
    VendrePoissons,
    ReparerBateau,
    AcheterVoilier,
    AcheterBaleinier,
    AcheterCanne,
    AcheterHamecon,
}

impl OptionBoutique {
    pub fn nom(&self) -> &str {
        match self {
            OptionBoutique::VendrePoissons => "Vendre tous ses poissons",
            OptionBoutique::ReparerBateau => "Réparer le bateau",
            OptionBoutique::AcheterVoilier => "Voilier",
            OptionBoutique::AcheterBaleinier => "Baleinier",
            OptionBoutique::AcheterCanne => "Canne à pêche",
            OptionBoutique::AcheterHamecon => "Hameçon",
        }
    }

    pub fn cout(&self) -> i32 {
        match self {
            OptionBoutique::VendrePoissons => 0,
            OptionBoutique::ReparerBateau => 1,
            OptionBoutique::AcheterVoilier => 100,
            OptionBoutique::AcheterBaleinier => 500,
            OptionBoutique::AcheterCanne => 130,
            OptionBoutique::AcheterHamecon => 175,
        }
    }

    pub fn emoji(&self) -> char {
        match self {
            OptionBoutique::VendrePoissons => '🐟',
            OptionBoutique::ReparerBateau => '🔨',
            OptionBoutique::AcheterVoilier => '⛵',
            OptionBoutique::AcheterBaleinier => '🚢',
            OptionBoutique::AcheterCanne => '🎣',
            OptionBoutique::AcheterHamecon => '🪝',
        }
    }

    pub fn action(&self, bateau: &mut Bateau) {
        match self {
            OptionBoutique::VendrePoissons => sell_all_poissons(bateau),
            OptionBoutique::ReparerBateau => repair_bateau(bateau),
            OptionBoutique::AcheterVoilier => buy_voilier(bateau),
            OptionBoutique::AcheterBaleinier => buy_baleinier(bateau),
            OptionBoutique::AcheterCanne => buy_canne(bateau),
            OptionBoutique::AcheterHamecon => buy_hamecon(bateau),
        }
    }
}

pub fn afficher_boutique(options_achetees: &Vec<bool>) {
    println!("Bienvenue dans le port !");

    // Affichage des catégories
    let options = vec![
        (OptionBoutique::VendrePoissons, "🐟 **Poissonnier**"),
        (OptionBoutique::ReparerBateau, "🛠️ **Calfat**"),
        (OptionBoutique::AcheterVoilier, "🛳️ **Forban**"),
        (OptionBoutique::AcheterBaleinier, "🛳️ **Forban**"),
        (OptionBoutique::AcheterCanne, "🧵 **Quincaillier**"),
        (OptionBoutique::AcheterHamecon, "🧵 **Quincaillier**"),
    ];

    // Boucle pour afficher les options
    for (index, (option, category)) in options.iter().enumerate() {
        // Affichage de la catégorie
        if index == 0 || index == 1 || index == 2 || index == 4 {
            println!("\n{}", category);
        }

        if !options_achetees[index] {
            // Si l'option est VendrePoissons ou ReparerBateau, ne pas afficher le prix
            if option == &OptionBoutique::VendrePoissons || option == &OptionBoutique::ReparerBateau {
                println!("   {}  | {} {}", index + 1, option.nom(), option.emoji());
            } else if option == &OptionBoutique::AcheterBaleinier && !options_achetees[2] {
                // Si l'option AcheterBaleinier et que AcheterVoilier n'a pas été acheté, ne pas afficher l'option
                continue;
            } else {
                // Sinon, afficher le prix avec l'emoji
                println!("   {}  | {} {} - {} 🪙", index + 1, option.nom(), option.emoji(), option.cout());
            }
        } else {
            // Si l'option a été achetée, afficher "Déjà acheté"
            println!("   {}  | {} {} - Déjà acheté", index + 1, option.nom(), option.emoji());
        }
    }
}


pub fn handle_boutique_input(bateau: &mut Bateau, input: char, options_achetees: &mut Vec<bool>) {
    bateau.status();
    println!("\nEntrez le numéro de votre choix : ");

    let option = match input {
        '1' => OptionBoutique::VendrePoissons,
        '2' => OptionBoutique::ReparerBateau,
        '3' => OptionBoutique::AcheterVoilier,
        '4' => {
            // Vérification si l'option 3 (AcheterVoilier) a été achetée
            if !options_achetees[2] {
                println!("Vous devez acheter un voilier avant de pouvoir acheter un baleinier.");
                return; // Empêche l'achat du baleinier si le voilier n'a pas été acheté
            }
            OptionBoutique::AcheterBaleinier
        },
        '5' => OptionBoutique::AcheterCanne,
        '6' => OptionBoutique::AcheterHamecon,
        _ => {
            println!("Choix invalide.");
            return;
        }
    };

    // Si l'option est VendrePoissons ou ReparerBateau, on les exécute immédiatement
    match option {
        OptionBoutique::VendrePoissons => {
            sell_all_poissons(bateau);
            return; // Fin de la fonction car l'action est effectuée
        },
        OptionBoutique::ReparerBateau => {
            repair_bateau(bateau);
            return; // Fin de la fonction car l'action est effectuée
        },
        _ => {} // Les autres options continueront la logique d'achat
    }

    // Si le joueur a suffisamment d'argent et n'a pas déjà acheté cette option
    if bateau.tresor >= option.cout() && !options_achetees[match option {
        OptionBoutique::AcheterVoilier => 2,
        OptionBoutique::AcheterBaleinier => 3,
        OptionBoutique::AcheterCanne => 4,
        OptionBoutique::AcheterHamecon => 5,
        _ => return,
    }] {
        println!("Vous avez choisi: {} {} | Coût: {} 🪙", option.nom(), option.emoji(), option.cout());
        bateau.tresor -= option.cout();
        option.action(bateau);

        // Marquer cette option comme achetée
        match option {
            OptionBoutique::AcheterVoilier => options_achetees[2] = true,
            OptionBoutique::AcheterBaleinier => options_achetees[3] = true,
            OptionBoutique::AcheterCanne => options_achetees[4] = true,
            OptionBoutique::AcheterHamecon => options_achetees[5] = true,
            _ => {}
        }
    } else if bateau.tresor < option.cout() {
        println!("Vous n'avez pas assez de 🪙 pour effectuer cette action.");
    } else {
        println!("Cette option a déjà été achetée.");
    }
}

fn sell_all_poissons(bateau: &mut Bateau) {
    let gain: f32 = bateau.cale.iter().map(|poisson| 0.03 * poisson.poids * poisson.taille).sum();
    bateau.tresor += gain as i32;
    bateau.cale.clear();
    println!("Vous avez gagné {} 🪙 en vendant vos poissons.", gain);
}

fn repair_bateau(bateau: &mut Bateau) {
    let cost_per_repair = 1;
    let repair_points = 5;
    let mut max_repairable = bateau.pv_max - bateau.pv;

    if max_repairable > 0 {
        while bateau.tresor >= cost_per_repair && max_repairable > 0 {
            let repair_cost = cost_per_repair;
            let repair_increment = std::cmp::min(repair_points, max_repairable);

            match bateau.heal(repair_increment) {
                Ok(_) => {
                    bateau.tresor -= repair_cost;
                    max_repairable -= repair_increment;
                }
                Err(e) => {
                    println!("Erreur pendant la réparation : {}", e);
                    break;
                }
            }
        }
        if bateau.pv >= bateau.pv_max {
            println!("Le bateau est complètement réparé.");
        } else {
            println!("Vous n'avez plus assez de 🪙 pour continuer la réparation.");
        }
    } else {
        println!("Le bateau est déjà à pleine santé.");
    }
}

fn buy_voilier(bateau: &mut Bateau) {
    bateau.emoji = '⛵';
    bateau.pv_max = 150;
    bateau.cale_initiale = bateau.cale_max;
    bateau.cale_max += 5;
}

fn buy_baleinier(bateau: &mut Bateau) {
    bateau.emoji = '🚢';
    bateau.pv_max = 200;
    bateau.cale_initiale = bateau.cale_max;
    bateau.cale_max += 5;
}

fn buy_canne(bateau: &mut Bateau) {
    bateau.cale_initiale = bateau.cale_max;
    bateau.cale_max += 1;
}

fn buy_hamecon(bateau: &mut Bateau) {
    bateau.cale_initiale = bateau.cale_max;
    bateau.cale_max += 3;
}

