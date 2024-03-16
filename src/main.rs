use std::{cmp, fmt, thread, time};

#[derive(Clone, Debug)]
enum Sector {
    Inhabited,
    Uninhabited,
}
impl fmt::Display for Sector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Sector::Inhabited => "⬜",
            Sector::Uninhabited => "⬛",
        };
        write!(f, "{}", printable)
    }
}

struct UniverseCreationError;
impl fmt::Display for UniverseCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This universe is invalid, try an alternate one.")
    }
}

struct Universe {
    seed_name: String,
    rows: usize,
    cols: usize,
    sectors: Vec<Vec<Sector>>,
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.sectors.iter() {
            for val in row.iter() {
                write!(f, "{}", val).expect("Error in printing Sector.");
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
            return Err(UniverseCreationError);
        }
        // Check to make sure all values are either 0 or 1, might as well make the grid on the fly too
        let mut row: usize = 0;
        let mut col: usize = 0;
        let mut grid_vec = vec![vec![Sector::Uninhabited; cols]; rows];
        for c in seed_vals.chars() {
            match c {
                '0' => grid_vec[row][col] = Sector::Uninhabited,
                '1' => grid_vec[row][col] = Sector::Inhabited,
                _ => return Err(UniverseCreationError),
            };
            col += 1;
            if col == cols {
                col = 0;
                row += 1;
            }
        }
        Ok(Universe {
            seed_name: name,
            rows: rows,
            cols: cols,
            sectors: grid_vec,
        })
    }

    fn process_cell(&self, row: usize, col: usize) -> Sector {
        let row_min = if row == 0 { row } else { row - 1 };
        let row_max = if row == self.rows - 1 { row } else { row + 1 };
        let col_min = if col == 0 { col } else { col - 1 };
        let col_max = if col == self.cols - 1 { col } else { col + 1 };

        let mut neighbors = 0;

        for i in row_min..=row_max {
            for j in col_min..=col_max {
                match self.sectors[i][j] {
                    Sector::Inhabited => neighbors += 1,
                    Sector::Uninhabited => (),
                };
            }
        }

        match self.sectors[row][col] {
            Sector::Inhabited => {
                neighbors -= 1;
                match neighbors {
                    ..=1 => Sector::Uninhabited,
                    2 | 3 => Sector::Inhabited,
                    _ => Sector::Uninhabited,
                }
            }
            Sector::Uninhabited => match neighbors {
                3 => Sector::Inhabited,
                _ => Sector::Uninhabited,
            },
        }
    }

    fn process_state(&self) -> Universe {
        let mut grid_vec = vec![vec![Sector::Uninhabited; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                grid_vec[i][j] = self.process_cell(i, j);
            }
        }

        Universe {
            seed_name: String::from("Hello"),
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
    let mut universe = match Universe::new(String::from("Testseed"), 5, 5, "0000000110011000010000000")
    {
        Ok(u) => u,
        Err(e) => panic!("{e}"),
    };

    println!("Seed:\n{universe}");

    // Run the universe
    let delay = time::Duration::from_millis(500);
    let mut epoch = 0;

    // Leave room for the board to change
    for _i in &universe.sectors {
        print!("\n");
    }
    loop {
        universe = universe.process_state();
        println!("Epoch: {epoch}");
        println!("{universe}");
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        epoch += 1;
        thread::sleep(delay);
    }
}
