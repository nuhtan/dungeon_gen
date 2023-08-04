// dungeon.rs

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Wall,
    Floor,
}

pub struct Dungeon {
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Dungeon {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::Wall; height]; width];
        Dungeon {
            grid,
            width,
            height,
        }
    }

    fn get_unvisited_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in &directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx < self.width && ny < self.height && self.grid[nx][ny] == Cell::Wall {
                neighbors.push((nx, ny));
            }
        }
        neighbors
    }

    fn carve_corridor(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.grid[x1][y1] = Cell::Floor;
        self.grid[x2][y2] = Cell::Floor;
    }

    pub fn recursive_backtracking(&mut self, x: usize, y: usize) {
        self.grid[x][y] = Cell::Floor;

        let mut neighbors = self.get_unvisited_neighbors(x, y);

        // Shuffle using the thread_rng() method from rand::Rng
        let mut rng = rand::thread_rng();
        neighbors.shuffle(&mut rng);
    
    }
}
