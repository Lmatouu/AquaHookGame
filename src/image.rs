use std::{thread, time::Duration};

pub fn afficher_ecran_chargement() -> Result<(), Box<dyn std::error::Error>> {
    let message = r#"
    _                     _   _             _    
   / \   __ _ _   _  __ _| | | | ___   ___ | | __
  / _ \ / _` | | | |/ _` | |_| |/ _ \ / _ \| |/ /
 / ___ \ (_| | |_| | (_| |  _  | (_) | (_) |   < 
/_/   \_\__, |\__,_|\__,_|_| |_|\___/ \___/|_|\_\
           |_|                                   

                     ____
                      ---|
          \/            /|     \/
                       / |\
                      /  | \        \/
                     /   || \
                    /    | | \
                   /     | |  \
                  /      | |   \
                 /       ||     \
                /        /       \
               /________/         \
               ________/__________--/
         ~~~   \___________________/
           ~~~      ~~~~~~~~~~      ~~~~~~
        ~~~~~~~~~~~~~~~~     ~~~~~~~~~
              ~~~~~~~                 ~~~~~~~
    "#;

    println!("{}", message);

    // Attendre 10 secondes avant de passer au jeu
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
