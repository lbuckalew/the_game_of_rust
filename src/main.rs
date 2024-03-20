use the_game_of_rust::seeds::get_seed;
use the_game_of_rust::seeds::get_seed_size;
use the_game_of_rust::seeds::search_seeds;
use the_game_of_rust::seeds::SeedError;
use the_game_of_rust::Universe;
use the_game_of_rust::seeds;

use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    println!(
        r#".___________. __    __   _______      _______      ___      .___  ___.  _______      ______    _______    .______       __    __       _______.___________.
|           ||  |  |  | |   ____|    /  _____|    /   \     |   \/   | |   ____|    /  __  \  |   ____|   |   _  \     |  |  |  |     /       |           |
`---|  |----`|  |__|  | |  |__      |  |  __     /  ^  \    |  \  /  | |  |__      |  |  |  | |  |__      |  |_)  |    |  |  |  |    |   (----`---|  |----`
    |  |     |   __   | |   __|     |  | |_ |   /  /_\  \   |  |\/|  | |   __|     |  |  |  | |   __|     |      /     |  |  |  |     \   \       |  |     
    |  |     |  |  |  | |  |____    |  |__| |  /  _____  \  |  |  |  | |  |____    |  `--'  | |  |        |  |\  \----.|  `--'  | .----)   |      |  |     
    |__|     |__|  |__| |_______|    \______| /__/     \__\ |__|  |__| |_______|    \______/  |__|        | _| `._____| \______/  |_______/       |__|    "#
    );
    println!("");

    let mut universe = Universe::new(String::from("def"), 2, 2, "....").unwrap();
    
    // Loop for finding a seed to start
    loop {
        println!("Find a seed! Do you want to");
        println!("(1) search for a seed or");
        println!("(2) start a seed's exact name");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        println!("");

        match answer.trim() {
            "1" => {
                println!("Okay, input a search term...");
                let mut search = String::new();
                std::io::stdin().read_line(&mut search).unwrap();
                search = search.trim().to_string();
                println!("");

                let search_results = search_seeds(search);
                match search_results {
                    Ok(s) => {
                        println!("Results:");
                        for res in s {
                            println!("{res}");
                        }
                        println!("");
                        continue;
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    } 
                }
            },
            "2" => {
                println!("Okay, input the exact seed name (without the .cells extension)...");
                let mut seed_name = String::new();
                std::io::stdin().read_line(&mut seed_name).unwrap();
                seed_name = seed_name.trim().to_string();

                let seed_result = get_seed_size(seed_name.clone());
                match seed_result {
                    Ok(seed_result) => {
                        println!("has a minimum grid size of {} rows and {} columns.", seed_result.0, seed_result.1);
                        println!("");
                        println!("How big of a grid would you like to place it in?");

                        println!("Rows:");
                        let mut num_rows = String::new();
                        std::io::stdin().read_line(&mut num_rows).unwrap();
                        let num_rows = num_rows.trim().parse().expect("number pls");

                        println!("Colums:");
                        let mut num_cols = String::new();
                        std::io::stdin().read_line(&mut num_cols).unwrap();
                        let num_cols = num_cols.trim().parse().expect("number pls");
                        println!("");

                        let seed_string = get_seed(seed_name, num_rows, num_cols).unwrap();

                        // Instantiate the universe
                        universe = Universe::new(String::from("Testseed"), num_rows, num_cols, &seed_string).unwrap();
                        println!("Seed:\n{universe}");
                        break;
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                }
            },
            _ => {
                println!("That selection isn't valid.");
                continue;
            }
        }
    }

    // Run the universe
    let delay = Duration::from_millis(500);
    let mut epoch = 0;

    for _i in &universe.sectors {
        print!("\n");
    }

    loop {
        let before = Instant::now();
        universe = universe.process_state();
        let elapsed = before.elapsed();

        println!("Epoch: {epoch} generated in {:}µs", elapsed.as_micros());
        println!("{universe}");
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        epoch += 1;

        sleep(delay);
    }
}
