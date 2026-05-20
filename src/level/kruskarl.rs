use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::level::edge::Edge;

struct UnionFind {
    parent_id: Uuid,
    rank: usize,
}

fn find(union_find: &HashMap<Uuid, UnionFind>, vertex: Uuid) -> Uuid {
    let mut current_vertex = vertex;
    while union_find[&current_vertex].parent_id != current_vertex {
        current_vertex = union_find[&current_vertex].parent_id;
    }
    current_vertex
}

fn union(union_find: &mut HashMap<Uuid, UnionFind>, v: Uuid, u: Uuid) {
    let root_v = find(union_find, v);
    let root_u = find(union_find, u);

    if root_v == root_u {
        return;
    }

    if union_find[&root_v].rank < union_find[&root_u].rank {
        union_find.get_mut(&root_v).unwrap().parent_id = root_u;
    } else if union_find[&root_v].rank > union_find[&root_u].rank {
        union_find.get_mut(&root_u).unwrap().parent_id = root_v;
    } else {
        union_find.get_mut(&root_u).unwrap().parent_id = root_v;
        union_find.get_mut(&root_v).unwrap().rank += 1;
    }
}

pub fn kruskarl(edge_list: Vec<Edge>) -> Vec<Edge> {
    let vertices = get_vertices(&edge_list);
    let mut tree_edges: Vec<Edge> = Vec::new();
    let mut union_find: HashMap<Uuid, UnionFind> = HashMap::new();

    for vertex in vertices {
        union_find.insert(
            vertex,
            UnionFind {
                parent_id: vertex,
                rank: 0,
            },
        );
    }

    // Edge list is already sorted by weight in ascending order
    for edge in edge_list {
        if find(&union_find, edge.get_from_id()) != find(&union_find, edge.get_to_id()) {
            union(&mut union_find, edge.get_from_id(), edge.get_to_id());
            tree_edges.push(edge);
        }
    }

    tree_edges
}

fn get_vertices(edge_list: &Vec<Edge>) -> HashSet<Uuid> {
    let mut vertices = HashSet::new();
    for edge in edge_list {
        vertices.insert(edge.get_from_id());
        vertices.insert(edge.get_to_id());
    }
    vertices
}

#[cfg(test)]
mod tests {
    use crate::level::{
        generate_edge_list::generate_edge_list,
        level_class::{Level, RoomData},
        room_file::RoomFile,
    };

    use super::*;

    #[test]
    fn test_kruskarl() {
        let small_room_file = RoomFile::from_file("rooms/small_room.json");
        let medium_room_file = RoomFile::from_file("rooms/medium_room.json");

        let room_data = vec![
            RoomData {
                room_file: small_room_file,
                count: 2,
            },
            RoomData {
                room_file: medium_room_file,
                count: 2,
            },
        ];

        let mut level = Level::new(100, 100);
        let rooms = level.place_rooms(room_data);
        let edge_list = generate_edge_list(rooms);
        assert_eq!(edge_list.len(), 96);

        let tree_edges = kruskarl(edge_list);
        assert_eq!(tree_edges.len(), 3);
    }
}
