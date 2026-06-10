use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use nalgebra::Vector2;

use crate::level::{edge::Edge, level::Level};

// Helper struct for binary heap
#[derive(Clone, Copy, Debug)]
struct State {
    cost: f32,
    distance_to_goal: f32,
    position: Vector2<usize>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let self_f_score = self.cost + self.distance_to_goal;
        let other_f_score = other.cost + other.distance_to_goal;

        self_f_score.total_cmp(&other_f_score).is_eq()
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_f_score = self.cost + self.distance_to_goal;
        let other_f_score = other.cost + other.distance_to_goal;

        // Reversed for min-heap (lowest total sum pops first)
        other_f_score.total_cmp(&self_f_score)
    }
}

impl Level {
    pub fn connect_corridors(&mut self, tree_edges: &Vec<Edge>) {
        for edge in tree_edges {
            self.create_corridor(edge.get_from(), edge.get_to());
        }
    }

    fn create_corridor(&mut self, from: Vector2<usize>, to: Vector2<usize>) {
        // A* algorithm
        let air_weight = (self.get_width() * self.get_height() + 1) as f32;
        let air_neighbour_weight = (self.get_width() * self.get_height() + 1) as f32;
        let normal_weight = 1.0;

        let wall_weight = f32::INFINITY;
        let mut visited: HashSet<Vector2<usize>> = HashSet::new();
        let mut parent_map: HashMap<Vector2<usize>, Vector2<usize>> = HashMap::new();
        let mut priority_queue = BinaryHeap::new();

        priority_queue.push(State {
            cost: 0.0,
            distance_to_goal: (to.cast::<f32>() - from.cast::<f32>()).norm(),
            position: from,
        });
        visited.insert(from);

        while let Some(state) = priority_queue.pop() {
            if state.position == to {
                self.build_corridor(parent_map, from, to);
                return;
            }

            for neighbour in self.get_neighbours(state.position) {
                if visited.contains(&neighbour) {
                    continue;
                }

                let mut tile_weight = match self.get_wall_tile(neighbour.x, neighbour.y) {
                    0 => air_weight,
                    -1 => normal_weight,
                    -2 => normal_weight,
                    _ => wall_weight,
                };
                for adj in self.get_neighbours(neighbour) {
                    if self.get_wall_tile(adj.x, adj.y) == 0 {
                        tile_weight += air_neighbour_weight;
                    }
                }

                let distance_to_goal = (to.cast::<f32>() - neighbour.cast::<f32>()).norm();
                let total_cost = state.cost + tile_weight;
                let new_state = State {
                    cost: total_cost,
                    distance_to_goal,
                    position: neighbour,
                };
                priority_queue.push(new_state);
                visited.insert(neighbour);
                parent_map.insert(neighbour, state.position);
            }
        }
    }

    fn build_corridor(
        &mut self,
        parent_map: HashMap<Vector2<usize>, Vector2<usize>>,
        from: Vector2<usize>,
        to: Vector2<usize>,
    ) {
        let mut current = to;
        while current != from {
            self.set_wall_map_tile(current.x, current.y, 0);
            current = parent_map[&current];
        }
    }

    fn get_neighbours(&self, position: Vector2<usize>) -> Vec<Vector2<usize>> {
        let mut neighbours = Vec::new();
        let directions = [
            Vector2::<isize>::new(0, 1),
            Vector2::new(0, -1),
            Vector2::new(1, 0),
            Vector2::new(-1, 0),
        ];

        for dir in directions.iter() {
            let nx = position.x as isize + dir.x;
            let ny = position.y as isize + dir.y;

            if nx >= 0 && ny >= 0 {
                let neighbour_pos = Vector2::new(nx as usize, ny as usize);
                if self.is_within_bounds(neighbour_pos) {
                    neighbours.push(neighbour_pos);
                }
            }
        }

        neighbours
    }

    fn is_within_bounds(&self, position: Vector2<usize>) -> bool {
        position.x < self.get_width() && position.y < self.get_height()
    }
}
