use crossterm::terminal::ClearType;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{stdout, IsTerminal, Write};
use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub fn draw_frame(string: &str, delay: Option<u64>) {
    let mut stdout = stdout();
    stdout.is_terminal();

    // Clear the terminal
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // Move the cursor to the top-left corner
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    println!("{string}");

    // Flush to ensure the output is displayed
    stdout.flush().unwrap();

    sleep(Duration::from_millis(delay.unwrap_or(9)));
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }

    pub fn plus(&self, x_offset: i64, y_offset: i64) -> Coord {
        Coord {
            x: self.x + x_offset,
            y: self.y + y_offset,
        }
    }

    pub fn plus_x(&self, offset: i64) -> Coord {
        self.plus(offset, 0)
    }

    pub fn plus_y(&self, offset: i64) -> Coord {
        self.plus(0, offset)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    grid: HashMap<Coord, T>,
    pub top_left: Coord,
    pub bottom_right: Coord,
}

impl<T> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid {
            grid: HashMap::new(),
            top_left: Coord::new(0, 0),
            bottom_right: Coord::new(0, 0),
        }
    }

    pub fn width(&self) -> i64 {
        (self.top_left.x - self.bottom_right.x).abs()
    }

    pub fn height(&self) -> i64 {
        (self.top_left.y - self.bottom_right.y).abs()
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        self.grid.insert(coord, value);
        self.bottom_right = Coord::new(
            self.bottom_right.x.max(coord.x),
            self.bottom_right.y.max(coord.y),
        );
        self.top_left = Coord::new(self.top_left.x.min(coord.x), self.top_left.y.min(coord.y));
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        self.grid.get(coord)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord, &T)> {
        self.grid.iter()
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = (0..=self.width())
            .map(|y| {
                let line = (0..=self.height())
                    .map(|x| {
                        self.get(&Coord { x, y })
                            .map(|value| format!("{:?}", value))
                            .unwrap_or(" ".to_string())
                    })
                    .collect::<String>();
                line + "\n"
            })
            .collect::<String>();

        write!(f, "{}", result)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Diagonal {
    TL,
    TR,
    BR,
    BL,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn new(letter: &char) -> Option<Self> {
        match letter {
            '^' => Some(Self::Up),
            '>' => Some(Self::Right),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            _ => None,
        }
    }
}
