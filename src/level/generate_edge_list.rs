use crate::level::{
    edge::{Door, Edge},
    room::Room,
};

pub fn generate_edge_list(room_list: Vec<Room>) -> Vec<Edge> {
    let mut edges = Vec::new();
    let mut doors: Vec<Door> = Vec::new();

    for room in room_list {
        let left_top = room.get_left_top_corner();
        let mut room_doors: Vec<Door> = Vec::new();
        for door_pos in room.file.get_door_positions() {
            room_doors.push(Door {
                room_id: room.get_id(),
                position: left_top + door_pos,
            });
        }

        for door in room_doors.iter() {
            for other_door in doors.iter() {
                edges.push(Edge::new(door.clone(), other_door.clone()));
            }
        }

        doors.extend(room_doors);
    }

    edges.sort_by(|a, b| a.get_weight().partial_cmp(&b.get_weight()).unwrap());
    edges
}
