use rand::prelude::random;
use std::{fmt::Display, time::Duration};

pub enum WrapMode {
    Wrap,
    NoWrap,
}

#[derive(Default, Clone, Copy)]
pub struct Cell(bool);

impl Cell {
    fn alive() -> Self {
        Cell(true)
    }

    fn dead() -> Self {
        Cell(false)
    }

    fn is_alive(&self) -> bool {
        self.0
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { '#' } else { ' ' })
    }
}

pub struct GameOfLife {
    width: usize,
    height: usize,
    field: Vec<Cell>,
    wrap: WrapMode,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize, wrap: WrapMode) -> Self {
        Self {
            width,
            height,
            field: GameOfLife::generate_field(width * height),
            wrap,
        }
    }
    pub fn update(&mut self) {
        self.field = self
            .field
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, c)| {
                let neighbors = self.count_neighbors(index);
                match neighbors {
                    2 => c,
                    3 => Cell::alive(),
                    _ => Cell::dead(),
                }
            })
            .collect()
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&Cell> {
        match self.wrap {
            WrapMode::Wrap => {
                let x = (x + self.width as isize) as usize % self.width;
                let y = (y + self.height as isize) as usize % self.height;
                let index = self.index(x, y);
                self.field.get(index)
            }
            WrapMode::NoWrap => {
                if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
                    None
                } else {
                    let index = self.index(x as usize, y as usize);
                    self.field.get(index)
                }
            }
        }
    }

    pub fn is_alive(&self, x: isize, y: isize) -> bool {
        self.get(x, y).map(|c| c.is_alive()).unwrap_or(false)
    }

    pub fn print_neighbors(&self) {
        self.field.iter().enumerate().for_each(|(i, _)| {
            if i > 0 && i % self.width == 0 {
                println!();
            }
            print!("{}", self.count_neighbors(i));
        })
    }

    fn generate_field(count: usize) -> Vec<Cell> {
        (0..count).map(|_| Cell(random::<bool>())).collect()
    }

    fn count_neighbors(&self, index: usize) -> usize {
        let (x, y) = self.index_to_coords(index);
        let x = x as isize;
        let y = y as isize;

        (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| (i, j)))
            .filter(|(i, j)| *i != 0 || *j != 0)
            .filter(|(i, j)| self.is_alive(x + i, y + j))
            .count()
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;

        (x, y)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl Display for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.field.iter().enumerate().try_for_each(|(i, c)| {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", c)
        })
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut game = GameOfLife::new(30, 30, WrapMode::Wrap);
    while game.field.iter().any(|c| c.is_alive()) {
        let start_time = std::time::Instant::now();
        clear_screen();
        println!("{}", game);
        game.update();
        let end_time = std::time::Instant::now();
        std::thread::sleep(
            Duration::from_secs_f32(1.0 / 30.0).saturating_sub(end_time - start_time),
        );
    }
    clear_screen();
    println!("{}", game);
}

#[cfg(test)]
mod tests {
    use crate::{GameOfLife, WrapMode};

    #[test]
    fn index_to_coords_test() {
        let game = GameOfLife::new(10, 5, WrapMode::NoWrap);

        assert_eq!(game.index_to_coords(0), (0, 0));
        assert_eq!(game.index_to_coords(1), (1, 0));
        assert_eq!(game.index_to_coords(5), (5, 0));
        assert_eq!(game.index_to_coords(10), (0, 1));
        assert_eq!(game.index_to_coords(49), (9, 4));
    }

    #[test]
    fn index_test() {
        let game = GameOfLife::new(10, 5, WrapMode::Wrap);
        assert_eq!(game.index(0, 0), 0);
        assert_eq!(game.index(1, 0), 1);
        assert_eq!(game.index(5, 0), 5);
        assert_eq!(game.index(0, 1), 10);
        assert_eq!(game.index(9, 4), 49);
    }
}
