use std::ops::Range;

const MAX_TRIES: u32 = 1000;
const PADDING: u32 = 5;

pub struct Floor {
    rooms: Option<Vec<Room>>,
    hallways: Option<Vec<Hallway>>,
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
    pub fn init(floor_dims: Vec2) -> Floor {
        return Floor {
            dimensions: floor_dims,
            rooms: None,
            hallways: None
        }
    }

    pub fn gen_rooms(
        &mut self,
        room_count_range: Range<u32>,
        room_dims_range: Range<u32>
    ) {
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
                    x: fastrand::u32(PADDING..self.dimensions.x - room_dims.x - PADDING),
                    y: fastrand::u32(PADDING..self.dimensions.y - room_dims.y - PADDING)
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

                if tries == MAX_TRIES {
                    break;
                }
            }
        }
        self.rooms = Some(rooms);
    }

    fn room_intersects_existing_rooms(
        room_loc: Vec2,
        room_dims: Vec2,
        rooms: &Vec<Room>
    ) -> bool {
        rooms.iter().any(|other_room| {
            (
                room_loc.x >= other_room.rel_loc.x - PADDING
                && room_loc.x <= other_room.rel_loc.x + other_room.dimensions.x + PADDING
                || room_loc.x + room_dims.x >= other_room.rel_loc.x - PADDING
                && room_loc.x + room_dims.x <= other_room.rel_loc.x + other_room.dimensions.x + PADDING
            ) && (
                room_loc.y >= other_room.rel_loc.y - PADDING
                && room_loc.y <= other_room.rel_loc.y + other_room.dimensions.y + PADDING
                || room_loc.y + room_dims.y >= other_room.rel_loc.y - PADDING
                && room_loc.y + room_dims.y <= other_room.rel_loc.y + other_room.dimensions.y + PADDING)
        })
    }

    pub fn point_within_room(
        &self,
        location: Vec2
    ) -> bool {
        if let Some(available_rooms) = &self.rooms {
            available_rooms.iter().any(|room| {
                (room.rel_loc.x..room.rel_loc.x + room.dimensions.x).contains(&location.x)
            && (room.rel_loc.y..room.rel_loc.y + room.dimensions.y).contains(&location.y)
            })
        } else {
            panic!("Rooms not yet initialized")
        }
    }
}