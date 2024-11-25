use crate::bateau::Bateau;

pub fn afficher_boutique() {
    println!("Bienvenue dans le port !");

    println!("\n🐟 **Poissonier**");
    println!("   1️⃣  | Vendre tous ses poissons");

    println!("\n🛠️  **Calfat**");
    println!("   2️⃣  | Réparation du bâteau - 1 🪙  pour 5 PV");

    println!("\n🛳️  **Forban**");
    println!("   3️⃣  | ⛵ Voilier - 100 🪙");
    println!("   4️⃣  | 🚢 Baleinier - 500 🪙");

    println!("\n🧵 **Quincaillier**");
    println!("   5️⃣  | 🎣 Canne à pêche - 130 🪙");
    println!("   6️⃣  | 🪝  Hameçon - 175 🪙");
}

pub fn handle_boutique_input(bateau: &mut Bateau, input: char) {
    bateau.status();
    println!("\nEntrez le numéro de votre choix : ");
    
    match input {
        '1' => {
            sell_all_poissons(bateau);
            println!("Tous les poissons ont été vendus.");
        }
        '2' => {
            repair_bateau(bateau);
        }
        '3' => {
            if bateau.tresor >= 100 {
                bateau.tresor -= 100;
                bateau.emoji = '⛵';
                bateau.pv_max = 150;
                bateau.cale_max = 9;
            }
        }
        '4' => {
            if bateau.tresor >= 500 {
                bateau.tresor -= 500;
                bateau.emoji = '🚢';
                bateau.pv_max = 200;
                bateau.cale_max = 13;
            }
        }
        '5' => {
            if bateau.tresor >= 130 {
                bateau.tresor -= 130;
            }
        }
        '6' => {
            if bateau.tresor >= 175 {
                bateau.tresor -= 175;
            }
        }
        _ => {
            println!("Choix invalide.");
        }
    }
}

pub fn sell_all_poissons(bateau: &mut Bateau) {
    let gain = bateau.cale * 25;
    bateau.tresor += gain;
    bateau.cale = 0;
    println!("Vous avez gagné {} 🪙 en vendant vos poissons.", gain);
}

pub fn repair_bateau(bateau: &mut Bateau) {
    let cost_per_repair = 1;  // Coût d'une réparation
    let repair_points = 5;    // Points de vie réparés à chaque réparation
    let mut max_repairable = bateau.pv_max - bateau.pv;  // Calcul du nombre de PV restants à réparer

    // Vérifie si le bateau peut être réparé
    if max_repairable > 0 {
        // Si le bateau peut être réparé et que le trésor est suffisant
        while bateau.tresor >= cost_per_repair && max_repairable > 0 {
            // Calcul du coût et de l'incrément de réparation
            let repair_cost = cost_per_repair;
            let repair_increment = std::cmp::min(repair_points, max_repairable);  // Répare jusqu'au maximum de PV restant

            // Effectuer la réparation en utilisant la méthode heal() de Bateau
            match bateau.heal(repair_increment) {
                Ok(_) => {
                    bateau.tresor -= repair_cost;  // Déduit le coût de la réparation du trésor
                    max_repairable -= repair_increment;  // Mets à jour le nombre de PV à réparer restant

                    println!(
                        "Réparé {} PV pour {} 🪙. Il vous reste {} PV à réparer et {} 🪙.",
                        repair_increment, repair_cost, max_repairable, bateau.tresor
                    );
                }
                Err(e) => {
                    println!("Erreur pendant la réparation : {}", e);
                    break;  // En cas d'erreur, on arrête la réparation
                }
            }
        }
        // Vérifier si le bateau est complètement réparé ou s'il manque d'argent
        if bateau.pv >= bateau.pv_max {
            println!("Le bateau est complètement réparé.");
        } else {
            println!("Vous n'avez plus assez de 🪙 pour continuer la réparation.");
        }
    } else {
        println!("Le bateau est déjà à pleine santé.");
    }
}