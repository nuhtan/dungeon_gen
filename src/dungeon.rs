use std::ops::Range;

/// The maximum number of tries when attempting to place a new room into a floor.
const MAX_TRIES: u32 = 1000;

/// The tiles that should pad the edges of the floor and the padding between potential rooms.
const PADDING: u32 = 5;

pub struct Floor {
    pub rooms: Option<Vec<Room>>,
    pub hallways: Option<Vec<Hallway>>,
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
        return Vec2 { x, y };
    }
}

pub struct Connection {
    pub starting_room: u32,
    pub ending_room: u32,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Segment {
    initial_location: Vec2,
    length: u32,
    direction: Direction,
}

pub struct Hallway {
    starting_point: Vec2,
    ending_point: Vec2,
    segments: Vec<Segment>,
}

pub struct Room {
    rel_loc: Vec2,
    dimensions: Vec2,
    pub index: u32,
}

impl Floor {
    pub fn init(floor_dims: Vec2) -> Floor {
        return Floor {
            dimensions: floor_dims,
            rooms: None,
            connections: None,
            hallways: None,
        };
    }

    pub fn gen_rooms(&mut self, room_count_range: Range<u32>, room_dims_range: Range<u32>) {
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
                    y: fastrand::u32(PADDING..self.dimensions.y - room_dims.y - PADDING),
                };
                let does_intersect =
                    Floor::room_intersects_existing_rooms(pos_loc, room_dims, &rooms);
                if !does_intersect {
                    rooms.push(Room {
                        dimensions: room_dims,
                        rel_loc: pos_loc,
                        index,
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

    fn room_intersects_existing_rooms(room_loc: Vec2, room_dims: Vec2, rooms: &Vec<Room>) -> bool {
        rooms.iter().any(|other_room| {
            (room_loc.x >= other_room.rel_loc.x - PADDING
                && room_loc.x <= other_room.rel_loc.x + other_room.dimensions.x + PADDING
                || room_loc.x + room_dims.x >= other_room.rel_loc.x - PADDING
                    && room_loc.x + room_dims.x
                        <= other_room.rel_loc.x + other_room.dimensions.x + PADDING)
                && (room_loc.y >= other_room.rel_loc.y - PADDING
                    && room_loc.y <= other_room.rel_loc.y + other_room.dimensions.y + PADDING
                    || room_loc.y + room_dims.y >= other_room.rel_loc.y - PADDING
                        && room_loc.y + room_dims.y
                            <= other_room.rel_loc.y + other_room.dimensions.y + PADDING)
        })
    }

    pub fn point_within_room(&self, location: Vec2) -> bool {
        if let Some(available_rooms) = &self.rooms {
            available_rooms.iter().any(|room| {
                (room.rel_loc.x..room.rel_loc.x + room.dimensions.x).contains(&location.x)
                    && (room.rel_loc.y..room.rel_loc.y + room.dimensions.y).contains(&location.y)
            })
        } else {
            panic!("Rooms not yet initialized")
        }
    }

    pub fn point_within_hallway(&self, location: Vec2) -> bool {
        if let Some(hallways) = &self.hallways {
            for hallway in hallways {
                for segment in &hallway.segments {
                    match segment.direction {
                        Direction::Up | Direction::Down => {
                            if location.x == segment.initial_location.x
                                && (segment.initial_location.y
                                    ..segment.initial_location.y + segment.length)
                                    .contains(&location.y)
                            {
                                return true;
                            }
                        }
                        _ => {
                            if location.y == segment.initial_location.y
                                && (segment.initial_location.x
                                    ..segment.initial_location.x + segment.length)
                                    .contains(&location.x)
                            {
                                return true;
                            }
                        }
                    }
                }
            }
            return false;
        } else {
            panic!("Hallways should be generated before attempting to render the floor")
        }
    }

    pub fn gen_connections(&mut self, extra_rooms_range: Range<u32>) {
        // Gets all of the rooms
        if let Some(rooms) = &self.rooms {
            let mut connections = Vec::new();

            // Create atleast one connection for each room except last.
            // Because we create a default connection between each room in the list all rooms can
            // be reached. Extra connections are attempted after the default connection is added.
            for room_num in 0..(rooms.len() - 1) {
                let current_room = &rooms[room_num];
                let next_room = &rooms[room_num + 1];
                connections.push(Connection {
                    starting_room: current_room.index,
                    ending_room: next_room.index,
                });
                for _ in 0..fastrand::u32(extra_rooms_range.clone()) {
                    connections.push(Connection {
                        starting_room: current_room.index,
                        ending_room: rooms[fastrand::usize(0..rooms.len())].index,
                    });
                }
            }
            self.connections = Some(connections);
        }
    }

    // Determines if a potential hallway would interesect any rooms on the floor
    fn segment_intersects_room(&self, starting_point: Vec2, target_point: Vec2) -> Option<Vec2> {
        if let Some(rooms) = &self.rooms {
            for room in rooms {
                if starting_point.x == target_point.x
                    && (room.rel_loc.x..room.rel_loc.x + room.dimensions.x)
                        .contains(&starting_point.x)
                    && (room.rel_loc.y <= starting_point.y.max(target_point.y)
                        && target_point.y.min(starting_point.y)
                            <= room.rel_loc.y + room.dimensions.y)
                {
                    if starting_point.y < target_point.y {
                        return Some(Vec2::new(starting_point.x, room.rel_loc.y - 2));
                    } else {
                        return Some(Vec2::new(
                            starting_point.y,
                            room.rel_loc.y + room.dimensions.y + 2,
                        ));
                    }
                } else if starting_point.y == target_point.y
                    && (room.rel_loc.y..room.rel_loc.y + room.dimensions.y)
                        .contains(&starting_point.y)
                    && (room.rel_loc.x <= starting_point.x.max(target_point.x)
                        && target_point.x.min(starting_point.x)
                            <= room.rel_loc.x + room.dimensions.x)
                {
                    if starting_point.x < target_point.x {
                        return Some(Vec2::new(room.rel_loc.x - 2, starting_point.y));
                    } else {
                        return Some(Vec2::new(
                            room.rel_loc.x + room.dimensions.x + 2,
                            starting_point.y,
                        ));
                    }
                }
            }
            return None;
        } else {
            panic!("Rooms must be generated before intersections can be checked")
        }
    }

    // Returns the world space location of a tile around a room that can be used to connect a
    // hallway to the room.
    fn get_room_adjacent_tile(&self, room: &Room) -> Vec2 {
        let direction = match fastrand::u32(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        };
        return if [Direction::Up, Direction::Down].contains(&direction) {
            Vec2::new(
                fastrand::u32(room.rel_loc.x..=room.rel_loc.x + room.dimensions.x),
                if direction == Direction::Down {
                    room.rel_loc.y - 1
                } else {
                    room.rel_loc.y + room.dimensions.y + 1
                },
            )
        } else {
            Vec2::new(
                if direction == Direction::Left {
                    room.rel_loc.x - 1
                } else {
                    room.rel_loc.x + room.dimensions.x + 1
                },
                fastrand::u32(room.rel_loc.y..=room.rel_loc.y + room.dimensions.y),
            )
        };
    }

    pub fn gen_hallways(&mut self) {
        // Get all of the rooms that have been generated
        if let Some(rooms) = &self.rooms {
            // Get all of the intended connections
            let mut hallways = Vec::new();
            if let Some(connections) = &self.connections {
                for connection in connections {
                    println!("Starting new connection");
                    let start_point =
                        self.get_room_adjacent_tile(&rooms[connection.starting_room as usize]);
                    let end_point =
                        self.get_room_adjacent_tile(&rooms[connection.ending_room as usize]);
                    let mut current_point = start_point;
                    let mut segments = Vec::new();

                    let mut last_dir = None;

                    while current_point != end_point {
                        println!(
                            "{},{} -> {},{}",
                            current_point.x, current_point.y, end_point.x, end_point.y
                        );
                        let mut target_point =
                            if i32::abs(current_point.x as i32 - end_point.x as i32)
                                > i32::abs(current_point.y as i32 - end_point.y as i32)
                                || last_dir
                                    .map_or(false, |true_if_horizontal| true_if_horizontal == false)
                            {
                                Vec2::new(end_point.x, current_point.y)
                            } else {
                                Vec2::new(current_point.x, end_point.y)
                            };

                        println!(
                            "Initial Target Point: {}, {}",
                            target_point.x, target_point.y
                        );

                        if let Some(early_terminating_point) =
                            self.segment_intersects_room(current_point, target_point)
                        {
                            target_point = early_terminating_point;
                        }

                        println!(
                            "Updated Target Point: {}, {}",
                            target_point.x, target_point.y
                        );

                        let l;
                        let d;

                        if current_point.x == target_point.x {
                            last_dir = Some(false);
                            l = i32::abs(current_point.y as i32 - target_point.y as i32);
                            d = if current_point.y < target_point.y {
                                Direction::Up
                            } else {
                                Direction::Down
                            };
                        } else {
                            last_dir = Some(true);
                            l = i32::abs(current_point.x as i32 - target_point.x as i32);
                            d = if current_point.x < target_point.x {
                                Direction::Right
                            } else {
                                Direction::Left
                            };
                        };

                        segments.push(Segment {
                            initial_location: current_point,
                            length: l as u32,
                            direction: d,
                        });
                        current_point = target_point;
                    }
                    hallways.push(Hallway {
                        starting_point: start_point,
                        ending_point: end_point,
                        segments,
                    });
                }
                self.hallways = Some(hallways);
            }
        }
    }
}
