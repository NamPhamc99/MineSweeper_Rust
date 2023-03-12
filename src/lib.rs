use std::collections::HashSet;
use rand::{thread_rng, Rng};

type Position = (u32, u32);

pub fn random_with_range (min: u32, max: u32) -> u32 {
    let mut rng = thread_rng();

    rng.gen_range(min..max)
}

struct Minesweeper {
    width: u32,
    height: u32,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>
}

impl Minesweeper {
    pub fn new (width: u32, height: u32, mine_count: usize) -> Minesweeper {
        let mut mines = HashSet::new();

        while mines.len() < mine_count {
            let coord_x = random_with_range(0, width);
            let coord_y = random_with_range(0, height);

            mines.insert((coord_x, coord_y));
        }

        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            // TODO: generate mines
            mines: Default::default(),
            flagged_fields: Default::default(),
        }
    }
}