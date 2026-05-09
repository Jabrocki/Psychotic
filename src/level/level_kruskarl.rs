use nalgebra::Vector2;

use crate::level::Level;

#[derive(Clone)]
pub struct Edge {
    from: Vector2<usize>,
    to: Vector2<usize>,
    weight: f32,
}

pub struct UnionFind {
    parent: Vector2<usize>,
    rank: i8,
}

impl Level {
    pub fn generate_edge_list(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        let mut nodes: Vec<Vector2<usize>> = Vec::new();

        for (room, left_top) in &self.rooms {
            let room_nodes: Vec<Vector2<usize>> = room
                .door_positions
                .iter()
                .map(|door_pos| *left_top + *door_pos)
                .collect();

            for room_node in room_nodes.iter().copied() {
                for other_node in nodes.iter().copied() {
                    let weight = (other_node.cast::<f32>() - room_node.cast::<f32>()).norm();

                    edges.push(Edge {
                        from: room_node,
                        to: other_node,
                        weight,
                    });
                }
            }

            nodes.extend(room_nodes);
        }

        edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
        edges
    }

    pub fn kruskarl(&self, edges: Vec<Edge>) -> Vec<Edge> {
        // Union-Find algorithm to connect doors inside rooms
        let vertices = self.get_vertices(&edges);
        let mut tree_edges = Vec::new();
        let mut union_find_map: HashMap<Vector2<usize>, UnionFind> = HashMap::new();
        for vertex in vertices {
            union_find_map.insert(
                vertex,
                UnionFind {
                    parent: vertex,
                    rank: 0,
                },
            );
        }

        for edge in edges {
            let from_parent = self.find(&union_find_map, edge.from);
            let to_parent = self.find(&union_find_map, edge.to);
            if from_parent != to_parent {
                tree_edges.push(edge.clone());
                self.union(&mut union_find_map, from_parent, to_parent);
            }
        }

        tree_edges
    }

    fn find(
        &self,
        union_find_map: &HashMap<Vector2<usize>, UnionFind>,
        vertex: Vector2<usize>,
    ) -> Vector2<usize> {
        let mut current_vertex = vertex;
        while union_find_map[&current_vertex].parent != current_vertex {
            current_vertex = union_find_map[&current_vertex].parent;
        }
        current_vertex
    }

    fn union(
        &self,
        union_find_map: &mut HashMap<Vector2<usize>, UnionFind>,
        vertex1: Vector2<usize>,
        vertex2: Vector2<usize>,
    ) {
        let root1 = self.find(union_find_map, vertex1);
        let root2 = self.find(union_find_map, vertex2);

        if root1 != root2 {
            if union_find_map[&root1].rank < union_find_map[&root2].rank {
                union_find_map.get_mut(&root1).unwrap().parent = root2;
            } else if union_find_map[&root1].rank > union_find_map[&root2].rank {
                union_find_map.get_mut(&root2).unwrap().parent = root1;
            } else {
                union_find_map.get_mut(&root2).unwrap().parent = root1;
                union_find_map.get_mut(&root1).unwrap().rank += 1;
            }
        }
    }

    fn get_vertices(&self, edges: &Vec<Edge>) -> HashSet<Vector2<usize>> {
        let mut vertices = HashSet::new();
        for edge in edges {
            vertices.insert(edge.from);
            vertices.insert(edge.to);
        }
        vertices
    }
}
