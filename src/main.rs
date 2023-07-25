mod dungeon;

use dungeon::{Floor, Vec2};

fn main() {
    let floor_dims = Vec2::new(100, 100);
    let floor = Floor::gen_floor(floor_dims, 3..8, 6..15);
    for y in 0..floor_dims.y {
        let mut line = "".to_string();
        for x in 0..floor_dims.x {
            line.push( match floor.point_within_room(Vec2::new(x, y)) {
                true => ' ',
                false => 'O'
            });
            
        }
        println!("{}", line);
    }
}
