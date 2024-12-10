#[derive(Clone, Debug)]
pub struct Grid<T>
where
    T: Default,
{
    pub x_size: usize,
    pub y_size: usize,
    grid: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone + Copy,
{
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            x_size,
            y_size,
            grid: vec![T::default(); x_size * y_size],
        }
    }

    pub fn from_bytes(data: &str) -> Grid<u8> {
        let lines = data.lines().collect::<Vec<_>>();
        assert!(!lines.is_empty());
        let x_size = lines[0].len();
        let y_size = lines.len();
        let grid = lines.iter().flat_map(|line| line.bytes()).collect();
        Grid::<u8> {
            x_size,
            y_size,
            grid,
        }
    }

    pub fn from_iter<I>(x_size: usize, y_size: usize, iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let grid = iter.collect();
        Self {
            x_size,
            y_size,
            grid,
        }
    }

    pub fn valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.x_size && pos.y >= 0 && (pos.y as usize) < self.y_size
    }

    pub fn at(&self, pos: &Position) -> Option<T> {
        if self.valid_pos(pos) {
            Some(self.grid[pos.y as usize * self.x_size + pos.x as usize])
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, pos: &Position) -> Option<&mut T> {
        if self.valid_pos(pos) {
            self.grid
                .get_mut(pos.y as usize * self.x_size + pos.x as usize)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.y_size).flat_map(|y| {
            (0..self.x_size)
                .map(move |x| Position::new(x.try_into().unwrap(), y.try_into().unwrap()))
        })
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn step(&self, dir: &Direction) -> Self {
        Self {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    pub fn rotate_90_cw(self) -> Self {
        match self {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!(),
        }
    }
}

pub const UP: Direction = Direction { x: 0, y: -1 };
pub const DOWN: Direction = Direction { x: 0, y: 1 };
pub const LEFT: Direction = Direction { x: -1, y: 0 };
pub const RIGHT: Direction = Direction { x: 1, y: 0 };

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];
