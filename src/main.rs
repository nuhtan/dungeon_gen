mod dungeon;

use dungeon::{Floor, Vec2};
use petgraph::{Undirected, Graph, dot::{Dot, Config}};

fn main() {
    let floor_dims = Vec2::new(300, 50);
    let mut floor = Floor::init(floor_dims);

    println!("Generating Rooms");
    floor.gen_rooms(10..20, 6..15);
    
    // for y in 0..floor_dims.y {
        // let mut line = "".to_string();
        // for x in 0..floor_dims.x {
            // line.push( match floor.point_within_room(Vec2::new(x, y)) {
                // true => ' ',
                // false => 'O'
            // });
            
        // }
        // println!("{}", line);
    // }

    println!("Generating Connections");
    floor.gen_connections(0..3);
    
    println!("Generating Hallways");
    floor.gen_hallways();

    // let mut g = Graph::<String, (), Undirected>::new_undirected();
    // let mut graph_rooms = Vec::new();
    // if let Some(rooms) = &mut floor.rooms {
        // for room in rooms {
            // graph_rooms.push(g.add_node(room.index.to_string()))
        // }
    // }
    // if let Some(connections) = &mut floor.connections {
        // for connection in connections {
            // g.add_edge(graph_rooms[connection.starting_room as usize], graph_rooms[connection.ending_room as usize], ());
        // }
    // }
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]))
}
