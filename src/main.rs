use std::fmt::{Display, Formatter, Result as FmtResult};
use std::thread::sleep;
use std::time::{Duration, Instant};

enum UniverseCreationError {
    InvalidSeedLength,
    InvalidSeedCharacter(char),
}
impl Display for UniverseCreationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "This universe is invalid, try an alternate one.").expect("_");
        match self {
            Self::InvalidSeedLength =>  write!(f, "The width or height provided is too small."),
            Self::InvalidSeedCharacter(c) => write!(f, "A bad character was provided ({c})."),
        }
    }
}

struct Universe {
    seed_name: String,
    seed_description: String,
    rows: usize,
    cols: usize,
    sectors: Vec<Vec<bool>>,
}
impl Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for row in self.sectors.iter() {
            for val in row.iter() {
                write!(f, "{}", if *val {"⬜"} else {"⬛"}).expect("Error in printing Sector.");
            }
            write!(f, "\n").expect("Error in printing Sector.");
        }
        write!(f, "")
    }
}

impl Universe {
    fn new(name: String, rows: usize, cols: usize, seed_vals: &str) -> Result<Universe, UniverseCreationError> {
        // Check if the seed_vals str is valid given the grid size and seed values
        if seed_vals.len() != rows * cols {
            return Err(UniverseCreationError::InvalidSeedLength);
        }
        // Check to make sure all values are either 0 or 1, might as well make the grid on the fly too
        let mut row: usize = 0;
        let mut col: usize = 0;
        let mut grid_vec = vec![vec![false; cols]; rows];
        for c in seed_vals.chars() {
            match c {
                '.' => grid_vec[row][col] = false,
                'O' => grid_vec[row][col] = true,
                c => return Err(UniverseCreationError::InvalidSeedCharacter(c)),

            };
            col += 1;
            if col == cols {
                col = 0;
                row += 1;
            }
        }
        Ok(Universe {
            seed_name: name,
            seed_description: String::from("TODO"),
            rows: rows,
            cols: cols,
            sectors: grid_vec,
        })
    }

    fn process_cell(&self, row: usize, col: usize) -> bool {
        let row_min = if row == 0 { row } else { row - 1 };
        let row_max = if row == self.rows - 1 { row } else { row + 1 };
        let col_min = if col == 0 { col } else { col - 1 };
        let col_max = if col == self.cols - 1 { col } else { col + 1 };

        let mut neighbors = 0;

        for i in row_min..=row_max {
            for j in col_min..=col_max {
                match self.sectors[i][j] {
                    true => neighbors += 1,
                    false => (),
                };
            }
        }

        match self.sectors[row][col] {
            true => {
                neighbors -= 1;
                match neighbors {
                    ..=1 => false,
                    2 | 3 => true,
                    _ => false,
                }
            }
            false => match neighbors {
                3 => true,
                _ => false,
            },
        }
    }

    fn process_state(&self) -> Universe {
        let mut grid_vec = vec![vec![false; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                grid_vec[i][j] = self.process_cell(i, j);
            }
        }

        Universe {
            seed_name: String::from("TODO name transfer"),
            seed_description: String::from("TODO description"),
            rows: self.rows,
            cols: self.cols,
            sectors: grid_vec,
        }
    }
}

fn main() {
    println!("Hello, world!\n");

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

    // Leave room for the board to change
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
