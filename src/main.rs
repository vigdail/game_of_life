use rand::prelude::random;
use std::{fmt::Display, time::Duration};

#[derive(Default, Clone, Copy)]
struct Cell(bool);

impl Cell {
    fn is_alive(&self) -> bool {
        self.0
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { '#' } else { ' ' })
    }
}

struct GameOfLife {
    width: usize,
    height: usize,
    field: Vec<Cell>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            field: GameOfLife::generate_field(width * height),
        }
    }
    pub fn update(&mut self) {
        self.field = self
            .field
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, c)| {
                let neighboars = self.count_neighboars(index);
                match neighboars {
                    2 => c,
                    3 => Cell(true),
                    _ => Cell(false),
                }
            })
            .collect()
    }

    pub fn is_alive(&self, x: isize, y: isize) -> bool {
        let x = (x + self.width as isize) as usize % self.width;
        let y = (y + self.height as isize) as usize % self.height;
        let index = self.index(x, y);
        self.field[index].is_alive()
    }

    #[allow(dead_code)]
    pub fn print_n(&self) {
        self.field.iter().enumerate().for_each(|(i, _)| {
            if i > 0 && i % self.width == 0 {
                println!("");
            }
            print!("{}", self.count_neighboars(i));
        })
    }

    fn generate_field(count: usize) -> Vec<Cell> {
        (0..count).map(|_| Cell(random::<bool>())).collect()
    }

    fn count_neighboars(&self, index: usize) -> usize {
        let (x, y) = self.index_to_coords(index);
        let x = x as isize;
        let y = y as isize;

        (-1..=1)
            .map(|i| (-1..=1).map(move |j| (i, j)))
            .flatten()
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
        self.field
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i > 0 && i % self.width == 0 {
                    writeln!(f, "")?;
                }
                write!(f, "{}", c)
            })
            .collect::<Result<_, _>>()
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut game = GameOfLife::new(5, 5);
    while game.field.iter().any(|c| c.is_alive()) {
        clear_screen();
        println!("{}", game);
        game.update();
        std::thread::sleep(Duration::from_millis(1000));
    }
    clear_screen();
    println!("{}", game);
}

#[cfg(test)]
mod tests {
    use crate::GameOfLife;

    #[test]
    fn index_to_coords_test() {
        let game = GameOfLife::new(10, 5);

        assert_eq!(game.index_to_coords(0), (0, 0));
        assert_eq!(game.index_to_coords(1), (1, 0));
        assert_eq!(game.index_to_coords(5), (5, 0));
        assert_eq!(game.index_to_coords(10), (0, 1));
        assert_eq!(game.index_to_coords(49), (9, 4));
    }

    #[test]
    fn index() {
        let game = GameOfLife::new(10, 5);
        assert_eq!(game.index(0, 0), 0);
        assert_eq!(game.index(1, 0), 1);
        assert_eq!(game.index(5, 0), 5);
        assert_eq!(game.index(0, 1), 10);
        assert_eq!(game.index(9, 4), 49);
    }
}
