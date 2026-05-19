use nalgebra::Vector2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::level::edge::Edge;
use crate::level::level_class::Level;

/*
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

impl Level {
    pub fn connect_corridors(&mut self, tree_edges: Vec<Edge>) {
        // A* algorithm to connect rooms with corridors
        // You shouldn't like to create corridors that are too long and have many intersections
        for edge in tree_edges {
            self.create_corridor(edge.get_from(), edge.get_to());
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
                if neighbor.x >= self.get_width() || neighbor.y >= self.get_height() {
                    continue;
                }

                if visited.contains(&neighbor) || self.get_wall_map()[neighbor.y][neighbor.x] != -1
                {
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
            self.set_wall_map_tile(current_position.x, current_position.y, 0); // Mark as air
            current_position = parent_map[&current_position];
        }
    }
}
*/
