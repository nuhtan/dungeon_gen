use std::ops::Range;

const MAX_TRIES: u32 = 1000;
const PADDING: u32 = 5;

pub struct Floor {
    rooms: Vec<Room>,
    hallways: Vec<Hallway>,
    dimensions: Vec2,
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

impl Vec2 {
    pub fn new(x: u32, y: u32) -> Vec2 {
        return Vec2 {
            x,
            y
        };
    }
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
    ) -> Floor {
        let mut rooms = Vec::new();
        let room_count = fastrand::u32(room_count_range);
        for _ in 0..room_count {
            let room_dims = Vec2 {
                x: fastrand::u32(room_dims_range.clone()),
                y: fastrand::u32(room_dims_range.clone()),
            };
            let mut placed = false;
            let mut tries = 0;
            while !placed {
                let pos_loc = Vec2 {
                    x: fastrand::u32(0..floor_dims.x - room_dims.x - PADDING),
                    y: fastrand::u32(0..floor_dims.y - room_dims.y - PADDING)
                };
                let does_intersect = Floor::room_intersects_existing_rooms(pos_loc, room_dims, &rooms);
                if !does_intersect {
                    rooms.push(Room {
                        dimensions: room_dims,
                        rel_loc: pos_loc,
                        connections: 0
                    });
                    placed = true;
                }
                tries += 1;

                if tries == 1000 {
                    break;
                }
            }
        }
        return Floor {
            dimensions: floor_dims,
            rooms: rooms,
            hallways: Vec::new()
        }
    }

    fn room_intersects_existing_rooms(
        room_loc: Vec2,
        room_dims: Vec2,
        rooms: &Vec<Room>
    ) -> bool {
        rooms.iter().any(|other_room| {
            (
                room_loc.x >= other_room.rel_loc.x - 1
                && room_loc.x <= other_room.rel_loc.x + other_room.dimensions.x + 1
                || room_loc.x + room_dims.x >= other_room.rel_loc.x - 1
                && room_loc.x + room_dims.x <= other_room.rel_loc.x + other_room.dimensions.x + 1
            ) && (
                room_loc.y >= other_room.rel_loc.y - 1
                && room_loc.y <= other_room.rel_loc.y + other_room.dimensions.y + 1
                || room_loc.y + room_dims.y >= other_room.rel_loc.y - 1
                && room_loc.y + room_dims.y <= other_room.rel_loc.y + other_room.dimensions.y + 1)
        })
    }

    pub fn point_within_room(
        &self,
        location: Vec2
    ) -> bool {
        self.rooms.iter().any(|room| {
            (room.rel_loc.x..room.rel_loc.x + room.dimensions.x).contains(&location.x)
                && (room.rel_loc.y..room.rel_loc.y + room.dimensions.y).contains(&location.y)
        })
    }

}