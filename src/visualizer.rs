// visualizer.rs

use crate::dungeon::{Dungeon, Cell};

pub struct Visualizer<'a> {
    dungeon: &'a Dungeon,
}

impl<'a> Visualizer<'a> {
    pub fn new(dungeon: &'a Dungeon) -> Self {
        Visualizer { dungeon }
    }

    pub fn visualize(&self) {
        for y in 0..self.dungeon.height {
            for x in 0..self.dungeon.width {
                match self.dungeon.grid[x][y] {
                    Cell::Wall => print!("#"),
                    Cell::Floor => print!("."),
                }
            }
            println!();
        }
    }
}
