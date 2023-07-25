use std::ops::Range;


pub struct Floor {
    rooms: Vec<Room>,
    hallways: Vec<Hallway>,
    dimensions: Vec2,
}

pub struct Vec2 {
    x: u32,
    y: u32,
}

struct Hallway {

}

struct Room {
    rel_loc: Vec2,
    dimensions: Vec2,
    connections: u32,
}

impl Floor {
    pub fn gen_floor(
        floor_dims: Vec2,
        room_count_range: Range<u32>,
        room_dims_range: Range<u32>
    ) -> () {
        let mut rooms = Vec::new();
        let room_count = fastrand::u32(room_count_range);
        for _ in 0..room_count {
            let room_dims = Vec2 {
                x: fastrand::u32(room_dims_range),
                y: fastrand::u32(room_dims_range),
            };
        }
    }
}