use the_game_of_rust::Universe;

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

    // Let them pick a seed
    println!("Pick a seed:");

    // Instantiate the universe
    let mut universe = match Universe::new(String::from("Testseed"), 12, 18, "....OO......OO.......O.O......O.O......O..........O...OO.O..........O.OOOO.O.O..OO..O.O.OO...O.O.O..O.O.O......O.O.O..O.O.O...OO.O.O..OO..O.O.OOOO.O..........O.OO...O..........O......O.O......O.O.......OO......OO....")
    {
        Ok(u) => u,
        Err(e) => panic!("{e}"),
    };
    println!("Seed:\n{universe}");

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
