use std::collections::{BinaryHeap, HashMap, HashSet};

use nalgebra::Vector2;
use rand::RngExt;

use std::cmp::Ordering;

use crate::level::Room;

pub struct RoomData {
    room: Room,
    count: usize,
}

pub struct Level {
    width: usize,
    height: usize,
    wall_map: Vec<Vec<i16>>,
    ceil_map: Vec<Vec<i16>>,
    floor_map: Vec<Vec<i16>>,
    rooms: Vec<(Room, Vector2<usize>)>, // (room, left_top_corner)
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            wall_map: vec![vec![-1; width]; height],
            ceil_map: vec![vec![-1; width]; height],
            floor_map: vec![vec![-1; width]; height],
            rooms: Vec::new(),
        }
    }

    pub fn generate(&mut self, room_data: Vec<RoomData>) {
        self.place_rooms(room_data);
        self.connect_rooms();
    }

    fn place_rooms(&mut self, room_data: Vec<RoomData>) {
        // Function to place rooms in the level
        for room_record in room_data {
            self.place_single_room(room_record);
        }
    }

    fn place_single_room(&mut self, mut room_record: RoomData) {
        // Function to place a single room in the level
        let mut rng = rand::rng();
        for _ in 0..room_record.count {
            let x = rng.random_range(0..self.width - room_record.room.width);
            let y = rng.random_range(0..self.height - room_record.room.height);
            // Check if the room can be placed at (x, y)
            if self.can_place_room(&room_record.room, x, y) {
                room_record = self.place_room_on_maps(room_record, x, y);
            }
        }
    }

    fn can_place_room(&self, room: &Room, x: usize, y: usize) -> bool {
        // Function to check if a room can be placed at (x, y)
        for i in 0..room.height {
            for j in 0..room.width {
                if self.wall_map[y + i][x + j] != -1 {
                    return false; // The space is not empty
                }
            }
        }
        true
    }

    fn place_room_on_maps(&mut self, room_record: RoomData, x: usize, y: usize) -> RoomData {
        // Function to place a room on the wall, ceil and floor maps
        for i in 0..room_record.room.height {
            for j in 0..room_record.room.width {
                self.wall_map[y + i][x + j] = room_record.room.wall_map[i][j];
                self.ceil_map[y + i][x + j] = room_record.room.ceil_map[i][j];
                self.floor_map[y + i][x + j] = room_record.room.floor_map[i][j];
            }
        }
        self.rooms
            .push((room_record.room.clone(), Vector2::new(x, y)));
        room_record
    }

    fn connect_rooms(&mut self) {
        // Function to connect rooms with corridors
        let edges = self.generate_edge_list();
        let tree_edges = self.kruskarl(edges);
        self.connect_corridors(tree_edges);
    }

    fn connect_corridors(&mut self, tree_edges: Vec<Edge>) {
        // A* algorithm to connect rooms with corridors
        // You shouldn't like to create corridors that are too long and have many intersections
        for edge in tree_edges {
            self.create_corridor(edge.from, edge.to);
        }
    }

    fn create_corridor(&mut self, from: Vector2<usize>, to: Vector2<usize>) {
        // Function to create a corridor between two points
        let mut priority_queue = BinaryHeap::new(); // This is max-heap
        let mut visited: HashSet<Vector2<usize>> = HashSet::new();
        let mut parent_map: HashMap<Vector2<usize>, Vector2<usize>> = HashMap::new();
        let start = QueueNode {
            position: from,
            cost: 0,
            heuristic: (to.cast::<f32>() - from.cast::<f32>()).norm() as i32,
        };
        priority_queue.push(start);
        parent_map.insert(from, from);
        visited.insert(from);

        while let Some(current_node) = priority_queue.pop() {
            if current_node.position == to {
                self.build_corridor(parent_map, from, to);
                break;
            }

            let neighbors = [
                Vector2::new(current_node.position.x + 1, current_node.position.y),
                Vector2::new(current_node.position.x - 1, current_node.position.y),
                Vector2::new(current_node.position.x, current_node.position.y + 1),
                Vector2::new(current_node.position.x, current_node.position.y - 1),
            ];

            for neighbor in neighbors {
                // Check is if is in the map
                if neighbor.x >= self.width || neighbor.y >= self.height {
                    continue;
                }

                if visited.contains(&neighbor) || self.wall_map[neighbor.y][neighbor.x] != -1 {
                    continue; // Skip visited or non-empty tiles
                }

                // Skip those tiles that are neighbouring to visited
            }
        }
    }

    fn build_corridor(
        &mut self,
        parent_map: HashMap<Vector2<usize>, Vector2<usize>>,
        from: Vector2<usize>,
        to: Vector2<usize>,
    ) {
        // Function to build a corridor based on the parent map
        let mut current_position = to;
        while current_position != from {
            self.wall_map[current_position.y][current_position.x] = 0; // Mark as air
            current_position = parent_map[&current_position];
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct QueueNode {
    position: Vector2<usize>,
    cost: i32,
    heuristic: i32,
}

impl QueueNode {
    fn priority(&self) -> i32 {
        -self.cost - self.heuristic
    }
}

// 1. PartialOrd is required by Ord
impl PartialOrd for QueueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 2. Ord implementation
impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // We use 'other.cmp(self)' instead of 'self.cmp(other)' to
        // flip the ordering, turning the Max-Heap into a Min-Heap.
        other
            .priority()
            .partial_cmp(&self.priority())
            .unwrap_or(Ordering::Equal)
    }
}
