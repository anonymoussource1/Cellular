use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use toml::Table;
use std::fs;

type Gen = Vec<bool>;
type Board = Vec<Gen>;
type Rule = [bool; 8];

#[derive(Debug)]
struct Simulation {
    board: Board,
    generation: usize,
    config: Table,
}

impl Simulation {
    fn new() -> Self {
        let length = 5;
        let config = fs::read_to_string("config.toml").expect("Failed to read config file").parse::<Table>().expect("Failed to read 'config.toml'"); 
        println!("{:?}", config);
        let generations = config.get("generations").expect("Failed to read field: generations").as_integer().expect("Not an integer!") as usize;

        let mut board = vec![vec![false; length]; generations];

        board[0][(length - 1) / 2] = true;

        Self {
            board: board,
            generation: 0,
            config: config,
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

    fn update_row(&mut self, r: usize) {
        for c in 0..self.board.get(r).expect("Failed to get row: {r}").len() {
            self.process_block(r, c);
        }
    }

    fn process_block(&mut self, r: usize, c: usize) {
        let rule = rules_to_bools(self.config.get("rule").expect("Failed to get field: rule").as_integer().expect("Not an integer!") as u8);
        self.board[r][c] = match self.get_block(r, c) {
            (true, true, true) => rule[0],
            (true, true, false) => rule[1],
            (true, false, true) => rule[2],
            (true, false, false) => rule[3],
            (false, true, true) => rule[4],
            (false, true, false) => rule[5],
            (false, false, true) => rule[6],
            (false, false, false) => rule[7],
        }
    }

    fn create(&mut self) {
        for r in 1..self.board.len() {
            self.update_row(r);

            let max_computing_length = self.config.get("max_computing_length").expect("Failed to get field: max_computing_length").as_integer().expect("Not an integer!") as usize;
            while self.board[0].len() < max_computing_length {
                let mut far_left = false;
                let mut far_right = false;
                if self.board[r][0] {
                    for row in self.board.iter_mut() {
                        row.insert(0, false);
                    }
                    far_left = true;
                }
                if self.board[r][self.board[0].len() - 1] {
                    for row in self.board.iter_mut() {
                        row.push(false);
                    }
                    far_right = true;
                }
                if far_left || far_right {
                    self.update_row(r);
                } else {
                    break;
                }
            }

            self.generation += 1;
        }
    }

    fn print(&self) {
        let max_drawing_length = self.config.get("max_drawing_length").expect("Failed to get field: max_drawing_length").as_integer().expect("Not an integer!") as usize;
        let num_to_cut = if self.board[0].len() - 2 > max_drawing_length {
            (self.board[0].len() - max_drawing_length) / 2
        } else {
            1
        };
        self.board.iter().for_each(|r| {
            r.iter().skip(num_to_cut).take(r.len() - 2 * num_to_cut).for_each(|c| {
                if *c {
                    print!("⬛");
                } else {
                    print!("⬜");
                }
            });
            println!();
        });
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        
        let max_drawing_length = self.config.get("max_drawing_length").expect("Failed to get field: max_drawing_length").as_integer().expect("Not an integer!") as usize;
        let num_to_cut = if self.board[0].len() - 2 > max_drawing_length {
            (self.board[0].len() - max_drawing_length) / 2
        } else {
            1
        };

        let cell_size = self.config.get("cell_size").expect("Failed to get field: cell_size").as_integer().expect("Not an integer!") as u32;

        self.board.iter().enumerate().for_each(|(r, row)| {
            row.iter().enumerate().skip(num_to_cut).take(row.len() - 2 * num_to_cut).for_each(|(c, col)| {
                if *col {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }

                canvas.fill_rect(Rect::new((c - num_to_cut) as i32 * cell_size as i32, r as i32 * cell_size as i32, cell_size, cell_size)).expect("Failed to fill rect");
            });
        });
    }
}

fn main() {
    let mut sim = Simulation::new();
    let sdl2_context = sdl2::init().expect("Failed to get sdl2 context");
    let video_subsystem = sdl2_context.video().expect("Failed to get video subsystem");
    let title = sim.config.get("name").expect("Failed to get field: title").as_str().expect("Not a str!");
    let width = sim.config.get("width").expect("Failed to get field: width").as_integer().expect("Not an integer!") as u32;
    let height = sim.config.get("height").expect("Failed to get field: height").as_integer().expect("Not an integer!") as u32;
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .expect("Failed to create canvas");
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .expect("Failed to create canvas");

    sim.create();
    sim.draw(&mut canvas);
    
    canvas.present();

    let mut event_pump = sdl2_context.event_pump().expect("Failed to get event pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
    }
}

fn rules_to_bools(rule: u8) -> Rule {
    format!("{rule:0>8b}").chars().into_iter().map(|c| if c == '1' { true } else { false }).collect::<Vec<bool>>().try_into().expect("Failed to covert Vec<bool> into Rule!")
}
