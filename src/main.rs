type Gen = Vec<bool>;
type Board = Vec<Gen>;

#[derive(Debug)]
struct Simulation {
    board: Board,
    generation: usize,
    rule: u8,
}

impl Simulation {
    fn new(generations: usize, rule: u8) -> Self {
        let length = 3;

        let mut board = vec![vec![false; length]; generations];

        board[0][(length - 1) / 2] = true;

        Self {
            board: board,
            generation: 0,
            rule: rule,
        }
    }

    fn get(&self, r: usize, c: usize) -> &bool {
        match self.board.get(r) {
            Some(row) => {
                match row.get(c) {
                    Some(cell) => cell,
                    None => &false,
                }
            }
            None => &false,
        }
    }

    fn get_block(&self, r: usize, c: usize) -> (&bool, &bool, &bool) {
        if c == 0 {
            (&false, self.get(r - 1, c), self.get(r - 1, c + 1))
        } else {
            (self.get(r - 1, c - 1), self.get(r - 1, c), self.get(r - 1, c + 1))
        }
    }

    fn update_col(&mut self, c: usize) {
        for r in 1..self.generation {
            self.process_block(r, c);
        }
    }

    fn update_row(&mut self, r: usize) {
        for c in 0..self.board.get(r).expect("Failed to get row: {r}").len() {
            self.process_block(r, c);
        }
        self.generation += 1;
    }

    fn process_block(&mut self, r: usize, c: usize) {
        self.board[r][c] = match self.get_block(r, c) {
            (true, true, true) => false,
            (true, true, false) => false,
            (true, false, true) => false,
            (true, false, false) => true,
            (false, true, true) => true,
            (false, true, false) => true,
            (false, false, true) => true,
            (false, false, false) => false,
        }
    }

    fn print(&self) {
        self.board.iter().for_each(|r| {
            r.iter().skip(1).take(r.len() - 2).for_each(|c| {
                if *c {
                    print!("⬛");
                } else {
                    print!("⬜");
                }
            });
            println!();
        });
    }
}

fn main() {
    let mut sim = Simulation::new(51, 255);
    for r in 1..sim.board.len() {
        sim.update_row(r);

        if sim.board[r][0] {
            for row in sim.board.iter_mut() {
                row.insert(0, false);
            }

            sim.update_col(0);
        }
        if sim.board[r][sim.board[0].len() - 1] {
            println!("yes");
            for row in sim.board.iter_mut() {
                row.push(false);
            }

            sim.update_col(sim.board[0].len() - 2);
        }
    }
    sim.print();
}
