// main.rs

mod dungeon;
mod visualizer;

use dungeon::Dungeon;
use visualizer::Visualizer;

fn main() {
    let width = 200;
    let height = 100;

    let mut dungeon = Dungeon::new(width, height);
    let start_x = 1;
    let start_y = 1;
    dungeon.recursive_backtracking(start_x, start_y);

    let visualizer = Visualizer::new(&dungeon);
    visualizer.visualize();
}
