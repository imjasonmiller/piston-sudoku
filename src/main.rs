use piston_window::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

struct Sudoku {
    data: Vec<Vec<u32>>,
    subgrid_len: usize,
    rng: ThreadRng,
}

impl Sudoku {
    pub fn new(size: usize) -> Sudoku {
        Self {
            data: vec![vec![0; size]; size],
            subgrid_len: (size as f32).sqrt() as usize,
            rng: thread_rng(),
        }
    }

    pub fn is_valid(&self, num: u32, row: usize, col: usize) -> bool {
        // Check the row
        if self.data[row].iter().any(|&value| value == num) {
            return false;
        }

        // Check the column
        if self.data.iter().any(|row| row[col] == num) {
            return false;
        }

        // Top left coordinates of the subgrid
        let subgrid_x = col / self.subgrid_len * self.subgrid_len;
        let subgrid_y = row / self.subgrid_len * self.subgrid_len;

        // Check subgrid
        for i in 0..self.subgrid_len {
            for j in 0..self.subgrid_len {
                if self.data[subgrid_y + i][subgrid_x + j] == num {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self, mut row: usize, mut col: usize) -> bool {
        if col == self.data.len() {
            row += 1;
            col = 0;
        }

        if row == self.data.len() {
            return true;
        }

        if self.data[row][col] != 0 {
            return self.solve(row, col + 1);
        }

        let mut numbers = (1..=self.data.len()).collect::<Vec<usize>>();
        numbers.shuffle(&mut self.rng);

        for n in numbers.into_iter() {
            if self.is_valid(n as u32, row, col) {
                self.data[row][col] = n as u32;
                if self.solve(row, col + 1) {
                    return true;
                }
            }
        }

        self.data[row][col] = 0;

        false
    }
}

fn main() {
    let title = "Sudoku generator";
    let mut window: PistonWindow = WindowSettings::new(title, [450, 450])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(1, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let mut sudoku = Sudoku::new(9);
    sudoku.solve(0, 0);

    let line = Line::new([0.0, 0.0, 0.0, 1.0], 1.0);
    let grid = grid::Grid {
        cols: 9,
        rows: 9,
        units: 50.0,
    };

    window.set_lazy(true);
    while let Some(event) = window.next() {
        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |context, graphics, device| {
                clear([1.0; 4], graphics);
                grid.draw(&line, &context.draw_state, context.transform, graphics);

                for (i, row) in sudoku.data.iter().enumerate() {
                    for (j, _col) in row.iter().enumerate() {
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 50)
                            .draw(
                                &sudoku.data[i][j].to_string(),
                                &mut glyphs,
                                &context.draw_state,
                                context
                                    .transform
                                    .trans(50.0 * i as f64 + 10.0, 50.0 * j as f64 + 50.0 - 7.5),
                                graphics,
                            )
                            .unwrap();
                    }
                }

                // Update glyphs before rendering
                glyphs.factory.encoder.flush(device);
            });
        }

        if event.press_args().is_some() {
            sudoku = Sudoku::new(9);
            sudoku.solve(0, 0);
        }
    }
}

