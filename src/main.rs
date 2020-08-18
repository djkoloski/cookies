use std::time::Instant;

// Change this to check different garden sizes
const GARDEN_SIDE: usize = 6;
const GARDEN_AREA: usize = GARDEN_SIDE * GARDEN_SIDE;

type Tiles = u64;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Garden {
    tiles: Tiles,
}

impl Garden {
    fn empty() -> Self {
        Self {
            tiles: 0,
        }
    }

    #[inline(always)]
    fn plant(&self, index: usize) -> Tiles {
        (self.tiles >> index) & 1
    }

    fn golden_clover_chance(&self) -> Tiles {
        const NEIGHBORS_SIDE: usize = GARDEN_SIDE + 2;
        const NEIGHBORS_AREA: usize = NEIGHBORS_SIDE * NEIGHBORS_SIDE;

        let mut neighbors = [0; NEIGHBORS_AREA];
        let mut i = 0;
        let mut n = NEIGHBORS_SIDE + 1;
        for _ in 0..GARDEN_SIDE {
            for _ in 0..GARDEN_SIDE {
                let plant = self.plant(i);
                neighbors[n + 1] += plant;
                neighbors[n - NEIGHBORS_SIDE + 1] += plant;
                neighbors[n - NEIGHBORS_SIDE] += plant;
                neighbors[n - NEIGHBORS_SIDE - 1] += plant;
                neighbors[n - 1] += plant;
                neighbors[n + NEIGHBORS_SIDE - 1] += plant;
                neighbors[n + NEIGHBORS_SIDE] += plant;
                neighbors[n + NEIGHBORS_SIDE + 1] += plant;
                i += 1;
                n += 1;
            }
            n += 2;
        }

        const NEIGHBORS_VALUE: [Tiles; 9] = [0, 0, 1, 1, 8, 7, 7, 7, 7];

        let mut total = 0;
        let mut i = 0;
        let mut n = NEIGHBORS_SIDE + 1;
        for _ in 0..GARDEN_SIDE {
            for _ in 0..GARDEN_SIDE {
                total += (1 - self.plant(i)) * NEIGHBORS_VALUE[neighbors[n] as usize];
                i += 1;
                n += 1;
            }
            n += 2;
        }

        total
    }

    fn advance(&mut self) {
        self.tiles += 1
    }

    fn print(&self) {
        for y in (0..GARDEN_SIDE).rev() {
            for x in 0..GARDEN_SIDE {
                let index = x + y * GARDEN_SIDE;
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
    let mut best = Garden::empty();
    let mut best_score = 0;

    let mut current = Garden::empty();

    let start = Instant::now();

    for _ in 0..(1u64 << GARDEN_AREA) {
        let score = current.golden_clover_chance();
        if score > best_score {
            best = current.clone();
            best_score = score;
        }
        current.advance();
    }

    println!("Time elapsed: {} seconds", Instant::now().duration_since(start).as_secs_f64());

    println!("Best score: {}", best_score);
    best.print();
}
