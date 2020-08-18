use std::time::Instant;
use rayon::prelude::*;

// Change this to check different garden sizes
const GARDEN_WIDTH: usize = 6;
const GARDEN_HEIGHT: usize = 6;
const GARDEN_AREA: usize = GARDEN_WIDTH * GARDEN_HEIGHT;

type Tiles = u64;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Garden {
    tiles: Tiles,
}

impl Garden {
    fn new(tiles: Tiles) -> Self {
        Self {
            tiles,
        }
    }

    #[inline(always)]
    fn plant(&self, index: usize) -> Tiles {
        (self.tiles >> index) & 1
    }

    #[inline(always)]
    fn mutation_chance(&self, neighbors_value: &[Tiles; 9]) -> Tiles {
        const NEIGHBORS_WIDTH: usize = GARDEN_WIDTH + 2;
        const NEIGHBORS_HEIGHT: usize = GARDEN_HEIGHT + 2;
        const NEIGHBORS_AREA: usize = NEIGHBORS_WIDTH * NEIGHBORS_HEIGHT;

        let mut neighbors = [0; NEIGHBORS_AREA];
        let mut i = 0;
        let mut n = NEIGHBORS_WIDTH + 1;
        for _ in 0..GARDEN_HEIGHT {
            for _ in 0..GARDEN_WIDTH {
                let plant = self.plant(i);
                neighbors[n + 1] += plant;
                neighbors[n - NEIGHBORS_WIDTH + 1] += plant;
                neighbors[n - NEIGHBORS_WIDTH] += plant;
                neighbors[n - NEIGHBORS_WIDTH - 1] += plant;
                neighbors[n - 1] += plant;
                neighbors[n + NEIGHBORS_WIDTH - 1] += plant;
                neighbors[n + NEIGHBORS_WIDTH] += plant;
                neighbors[n + NEIGHBORS_WIDTH + 1] += plant;
                i += 1;
                n += 1;
            }
            n += 2;
        }

        let mut total = 0;
        let mut i = 0;
        let mut n = NEIGHBORS_WIDTH + 1;
        for _ in 0..GARDEN_HEIGHT {
            for _ in 0..GARDEN_WIDTH {
                total += (1 - self.plant(i)) * neighbors_value[neighbors[n] as usize];
                i += 1;
                n += 1;
            }
            n += 2;
        }

        total
    }

    #[inline(always)]
    pub fn golden_clover_chance(&self) -> Tiles {
        const NEIGHBORS_VALUE: [Tiles; 9] = [0, 0, 1, 1, 8, 7, 7, 7, 7];
        self.mutation_chance(&NEIGHBORS_VALUE)
    }

    #[inline(always)]
    pub fn mutation_chance_2_parents(&self) -> Tiles {
        const NEIGHBORS_VALUE: [Tiles; 9] = [0, 0, 1, 1, 1, 1, 1, 1, 1];
        self.mutation_chance(&NEIGHBORS_VALUE)
    }

    pub fn print(&self) {
        for y in (0..GARDEN_HEIGHT).rev() {
            for x in 0..GARDEN_WIDTH {
                let index = x + y * GARDEN_WIDTH;
                let c = match self.plant(index) {
                    0 => '.',
                    _ => 'X',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn main() {
    let start = Instant::now();

    let best = (0..(1u64 << GARDEN_AREA)).into_par_iter().max_by_key(|&n| {
        let current = Garden::new(n);
        current.golden_clover_chance()
        //current.mutation_chance_2_parents()
    });

    println!("Time elapsed: {} seconds", Instant::now().duration_since(start).as_secs_f64());

    if let Some(best) = best {
        let best = Garden::new(best);
        let best_score = best.mutation_chance_2_parents();

        println!("Best score: {}", best_score);
        println!();
        best.print();
    } else {
        println!("No best configuration");
    }
}
