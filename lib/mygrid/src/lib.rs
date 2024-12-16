use std::ops::*;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub x_size: usize,
    pub y_size: usize,
    grid: Vec<T>,
}

impl<T> Index<&Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: &Position) -> &Self::Output {
        self.at(pos).unwrap()
    }
}

impl<T> IndexMut<&Position> for Grid<T> {
    fn index_mut(&mut self, pos: &Position) -> &mut Self::Output {
        self.at_mut(pos).unwrap()
    }
}

impl<T> Grid<T> {
    pub fn new(x_size: usize, y_size: usize) -> Self
    where
        T: Default + Clone,
    {
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

    pub fn fill(&mut self, v: T)
    where
        T: Clone,
    {
        self.grid.fill(v);
    }

    pub fn find(&self, t: &T) -> Option<Position>
    where
        T: PartialEq,
    {
        self.grid
            .iter()
            .enumerate()
            .find(|&(_, v)| v == t)
            .and_then(|(i, _)| {
                Some(Position::new(
                    (i % self.x_size) as i32,
                    (i / self.x_size) as i32,
                ))
            })
    }

    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if i != 0 && i % self.x_size == 0 {
                    "\n".to_string() + &v.to_string()
                } else {
                    v.to_string()
                }
            })
            .collect()
    }

    pub fn valid_pos(&self, pos: &Position) -> bool {
        pos.x >= 0 && (pos.x as usize) < self.x_size && pos.y >= 0 && (pos.y as usize) < self.y_size
    }

    pub fn at(&self, pos: &Position) -> Option<&T> {
        if self.valid_pos(pos) {
            Some(&self.grid[pos.y as usize * self.x_size + pos.x as usize])
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

    pub fn iter_region<'a, F>(
        &'a self,
        start: &Position,
        same_region_fn: F,
    ) -> impl Iterator<Item = (Position, &'a T)>
    where
        F: Fn(Position, &'a T) -> bool + Copy,
    {
        RegionIterator::new(self, start, same_region_fn)
    }

    pub fn iter_neighbors<'a>(
        &'a self,
        start: &Position,
    ) -> impl Iterator<Item = (Position, &'a T)> {
        let start = start.clone();
        CARDINAL_DIRECTIONS
            .iter()
            .map(move |dir| start.step(dir))
            .filter_map(|pos| self.at(&pos).and_then(|v| Some((pos, v))))
    }

    pub fn iter_neighbor_positions(&self, start: &Position) -> impl Iterator<Item = Position> {
        let start = start.clone();
        CARDINAL_DIRECTIONS.iter().map(move |dir| start.step(dir))
    }

    pub fn iter_surrounding_positions(&self, start: &Position) -> impl Iterator<Item = Position> {
        let start = start.clone();
        ALL_DIRECTIONS.iter().map(move |dir| start.step(dir))
    }
}

struct RegionIterator<'a, T, F>
where
    F: Fn(Position, &'a T) -> bool,
{
    grid: &'a Grid<T>,
    visited: Grid<bool>,
    pending: Vec<(Position, &'a T)>,
    same_region_fn: F,
}

impl<'a, T, F: Fn(Position, &'a T) -> bool> RegionIterator<'a, T, F> {
    fn new(grid: &'a Grid<T>, start: &Position, same_region_fn: F) -> Self {
        Self {
            grid,
            visited: Grid::<bool>::new(grid.x_size, grid.y_size),
            pending: vec![(start.clone(), &grid.at(start).expect("valid"))],
            same_region_fn: same_region_fn,
        }
    }

    fn same_region_neighbors(
        grid: &'a Grid<T>,
        same_region_fn: F,
        start: &Position,
    ) -> impl Iterator<Item = (Position, &'a T)> {
        grid.iter_neighbors(start)
            .filter(move |&(pos, v)| same_region_fn(pos, &v))
    }
}

impl<'a, T, F: Fn(Position, &'a T) -> bool> Iterator for RegionIterator<'a, T, F>
where
    T: 'a,
    F: Copy,
{
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (pos, v) = self.pending.pop()?;
        *self.visited.at_mut(&pos).expect("valid") = true;
        let new_region_neighbors =
            Self::same_region_neighbors(self.grid, self.same_region_fn, &pos);
        for (neighbor, v) in new_region_neighbors {
            let Some(visited) = self.visited.at_mut(&neighbor) else {
                continue;
            };
            if visited == &true {
                continue;
            }
            *visited = true;
            self.pending.push((neighbor, v));
        }
        Some((pos, v))
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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

#[auto_impl_ops::auto_ops]
impl AddAssign<&Position> for Position {
    fn add_assign(&mut self, other: &Self) {
        self.x = &self.x + &other.x;
        self.y = &self.y + &other.y;
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&i32> for Position {
    fn add_assign(&mut self, other: &i32) {
        self.x = &self.x + other;
        self.y = &self.y + other;
    }
}

#[auto_impl_ops::auto_ops]
impl MulAssign<&Position> for Position {
    fn mul_assign(&mut self, other: &Self) {
        self.x = &self.x * &other.x;
        self.y = &self.y * &other.y;
    }
}

#[auto_impl_ops::auto_ops]
impl MulAssign<&i32> for Position {
    fn mul_assign(&mut self, other: &i32) {
        self.x = &self.x * other;
        self.y = &self.y * other;
    }
}

#[auto_impl_ops::auto_ops]
impl DivAssign<&i32> for Position {
    fn div_assign(&mut self, other: &i32) {
        self.x = &self.x / other;
        self.y = &self.y / other;
    }
}

#[auto_impl_ops::auto_ops]
impl RemAssign<&Position> for Position {
    fn rem_assign(&mut self, other: &Self) {
        self.x = &self.x % &other.x;
        self.y = &self.y % &other.y;
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

    pub fn from(c: u8) -> Self {
        match c {
            b'^' => UP,
            b'>' => RIGHT,
            b'v' => DOWN,
            b'<' => LEFT,
            _ => panic!(),
        }
    }

    pub fn opposite(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub const UP: Direction = Direction { x: 0, y: -1 };
pub const DOWN: Direction = Direction { x: 0, y: 1 };
pub const LEFT: Direction = Direction { x: -1, y: 0 };
pub const RIGHT: Direction = Direction { x: 1, y: 0 };

pub const CARDINAL_DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];

pub const UP_LEFT: Direction = Direction { x: -1, y: -1 };
pub const DOWN_LEFT: Direction = Direction { x: -1, y: 1 };
pub const UP_RIGHT: Direction = Direction { x: 1, y: -1 };
pub const DOWN_RIGHT: Direction = Direction { x: 1, y: 1 };

pub const DIAGONAL_DIRECTIONS: [Direction; 4] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

pub const ALL_DIRECTIONS: [Direction; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];
