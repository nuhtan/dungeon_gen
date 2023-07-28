use std::{ops::Range};

/// The maximum number of tries when attempting to place a new room into a floor.
const MAX_TRIES: u32 = 1000;

/// The tiles that should pad the edges of the floor and the padding between potential rooms.
const PADDING: u32 = 5;

pub struct Floor {
    pub rooms: Option<Vec<Room>>,
    _hallways: Option<Vec<Hallway>>,
    pub connections: Option<Vec<Connection>>,
    dimensions: Vec2,
}

#[derive(Copy, Clone, PartialEq)]
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

pub struct Connection {
    pub starting_room: u32,
    pub ending_room: u32
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Segment {
    initial_location: Vec2,
    length: u32,
    direction: Direction
}

struct Hallway {
    starting_point: Vec2,
    ending_point: Vec2,
    segments: Vec<Segment>
}

pub struct Room {
    rel_loc: Vec2,
    dimensions: Vec2,
    pub index: u32
}

impl Floor {
    pub fn init(floor_dims: Vec2) -> Floor {
        return Floor {
            dimensions: floor_dims,
            rooms: None,
            connections: None,
            _hallways: None
        }
    }

    pub fn gen_rooms(
        &mut self,
        room_count_range: Range<u32>,
        room_dims_range: Range<u32>
    ) {
        let mut rooms = Vec::new();
        let room_count = fastrand::u32(room_count_range);
        for index in 0..room_count {
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
                        index
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

    pub fn gen_connections(
        &mut self,
        extra_rooms_range: Range<u32>
    ) {
        if let Some(rooms) = &self.rooms {
            let mut connections = Vec::new();
            for room_num in 0..(rooms.len() - 1) {
                let current_room = &rooms[room_num];
                let next_room = &rooms[room_num + 1];
                connections.push(Connection {
                    starting_room: current_room.index,
                    ending_room: next_room.index
                });
                for _ in 0..fastrand::u32(extra_rooms_range.clone()) {
                    connections.push(Connection {
                        starting_room: current_room.index,
                        ending_room: rooms[fastrand::usize(0..rooms.len())].index
                    });
                }
            }
            self.connections = Some(connections);
        }
    }

    fn hallway_intersects_room(
        &self,
        starting_point: Vec2,
        target_point: Vec2,
    ) -> Option<Vec2> {
        unimplemented!("None or point that should be used for additional segment")
    }

    fn get_room_adjacent_tile(&self, room: &Room) -> Vec2 {
        unimplemented!("Not yet")
    }

    pub fn gen_hallways(
        &mut self
    ) {
        if let Some(rooms) = &self.rooms {
            if let Some(connections) = &self.connections {
                for connection in connections {
                    let start_point = self.get_room_adjacent_tile(&rooms[connection.starting_room as usize]);
                    let end_point = self.get_room_adjacent_tile(&rooms[connection.ending_room as usize]);
                    let current_point = start_point;
                    while current_point != end_point {
                        if i32::abs(current_point.x as i32 - end_point.x as i32) > i32::abs(current_point.y as i32 - end_point.y as i32) {
                             
                         } else {
                            
                        }
                    }
                }
            }
        }
    }
}
