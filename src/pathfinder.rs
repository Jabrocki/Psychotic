use std::{
    alloc::{Layout, alloc, dealloc},
    collections::BinaryHeap,
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

use nalgebra::Vector2;

use crate::world::World;

struct Node {
    x: i32,
    y: i32,
    parent: NodePtr,

    f: i32,
    g: i32,
    h: i32,
}

impl Node {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            parent: NodePtr::nullptr(),
            f: 0,
            g: 0,
            h: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct NodePtr {
    ptr: *mut Node,
    layout: Layout,
}

impl NodePtr {
    fn new(node: Node) -> Self {
        let layout = Layout::new::<Node>();

        let data: *mut u8 = unsafe { alloc(layout) }; // malloc
        let data_t: *mut Node = data as *mut Node;

        unsafe {
            std::ptr::write(data_t, node);
        }

        NodePtr {
            ptr: data_t,
            layout: layout,
        }
    }

    fn nullptr() -> Self {
        let layout = Layout::new::<Node>();

        NodePtr {
            ptr: std::ptr::null_mut(),
            layout: layout,
        }
    }

    fn destroy(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

impl DerefMut for NodePtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return unsafe { &mut *self.ptr };
    }
}

impl Deref for NodePtr {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        return unsafe { &*self.ptr };
    }
}

impl PartialEq for NodePtr {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for NodePtr {}

impl Ord for NodePtr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for NodePtr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

pub fn find_path(gt_x: i32, gt_y: i32, x: i32, y: i32, world: &World) -> Option<Vec<Vector2<i32>>> {
    let mut alloced_nodes: Vec<NodePtr> = vec![];

    let mut open: BinaryHeap<NodePtr> = BinaryHeap::new();
    let mut closed: Vec<NodePtr> = vec![];

    let start = NodePtr::new(Node::new(x, y));
    let goal = NodePtr::new(Node::new(gt_x, gt_y));

    alloced_nodes.push(start);
    alloced_nodes.push(goal);

    open.push(start);

    while let Some(q) = open.pop() {
        let mut successors: Vec<NodePtr> = vec![];

        successors.push(NodePtr::new(Node::new(q.x + 1, q.y)));
        successors.push(NodePtr::new(Node::new(q.x - 1, q.y)));

        successors.push(NodePtr::new(Node::new(q.x, q.y + 1)));
        successors.push(NodePtr::new(Node::new(q.x, q.y - 1)));

        successors.push(NodePtr::new(Node::new(q.x + 1, q.y + 1)));
        successors.push(NodePtr::new(Node::new(q.x + 1, q.y - 1)));

        successors.push(NodePtr::new(Node::new(q.x - 1, q.y + 1)));
        successors.push(NodePtr::new(Node::new(q.x - 1, q.y - 1)));

        for &s in &successors {
            alloced_nodes.push(s);
        }

        for mut successor in successors {
            if successor == goal {
                closed.push(q);

                successor.parent = q;

                // backtrack

                let mut path: Vec<Vector2<i32>> = vec![];

                path.push(Vector2::new(goal.x, goal.y));

                while successor.parent.ptr != null_mut() {
                    if successor.x == start.x && successor.y == start.y {
                        break;
                    }
                    path.push(Vector2::new(successor.parent.x, successor.parent.y));
                    successor = successor.parent;
                }

                path.reverse();

                // deallocate memory

                for v in &mut alloced_nodes {
                    v.destroy();
                }

                return Some(path);
            }

            successor.g = q.g + 1;
            successor.h = (successor.x - goal.x) * (successor.x - goal.x)
                + (successor.y - goal.y) * (successor.y - goal.y);
            successor.f = successor.g + successor.h;
            successor.parent = q;

            if closed.contains(&successor) || open.iter().find(|x| **x == successor).is_some() {
                continue;
            }

            if successor.x < 0 || successor.y < 0 {
                continue;
            }

            if world.get_wall_tile(successor.x as usize, successor.y as usize) == 0 {
                open.push(successor);
            }
        }

        closed.push(q);
    }

    for v in &mut alloced_nodes {
        v.destroy();
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_empty_world(width: usize, height: usize) -> World {
        let wall_map = vec![vec![0; width]; height];
        World::new(wall_map, vec![], vec![])
    }

    #[test]
    fn test_straight_line_path() {
        let world = create_empty_world(5, 5);

        let path = find_path(4, 0, 0, 0, &world);

        assert!(path.is_some());
        let path_vec = path.unwrap();

        assert_eq!(
            path_vec.first().unwrap(),
            &Vector2::new(0, 0),
        );
        assert_eq!(
            path_vec.last().unwrap(),
            &Vector2::new(4, 0),
        );
        assert_eq!(
            path_vec.len(),
            5,
        );
    }

    #[test]
    fn test_blocked_path_returns_none() {
        let wall_map = vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        let world = World::new(wall_map, vec![], vec![]);

        let path = find_path(2, 1, 0, 1, &world);

        assert!(
            path.is_none(),
        );
    }

    #[test]
    fn test_navigate_around_obstacle() {
        let wall_map = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let world = World::new(wall_map, vec![], vec![]);

        let path = find_path(2, 2, 0, 0, &world);

        assert!(
            path.is_some(),
        );
    }

    #[test]
    fn test_already_at_destination() {
        let world = create_empty_world(3, 3);

        let path = find_path(1, 1, 1, 1, &world);

        assert!(path.is_some());
    }

    #[test]
    fn test_maze_dead_end() {
        let wall_map = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 1],
            vec![0, 1, 0, 1],
            vec![0, 1, 1, 1],
        ];
        let world = World::new(wall_map, vec![], vec![]);

        let path = find_path(2, 2, 0, 0, &world);
        assert!(
            path.is_none(),
        );
    }

    #[test]
    fn test_negative_coordinates_protection() {
        let world = create_empty_world(5, 5);

        let path = find_path(1, 1, 0, 0, &world);
        assert!(
            path.is_some(),
        );
    }

    #[test]
    fn test_maze_1() {
        let wall_map = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
        ];
        let world = World::new(wall_map, vec![], vec![]);

        let path = find_path(4, 4, 0, 0, &world);

        assert!(
            path.is_some(),
        );
        let path_vec = path.unwrap();

        assert_eq!(path_vec.first().unwrap(), &Vector2::new(0, 0));
        assert_eq!(path_vec.last().unwrap(), &Vector2::new(4, 4));

        for node in &path_vec {
            assert_eq!(
                world.get_wall_tile(node.x as usize, node.y as usize),
                0
            );
        }
    }

    #[test]
    fn test_maze_2() {
        let wall_map = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];
        let world = World::new(wall_map, vec![], vec![]);

        let path = find_path(5, 3, 0, 3, &world);

        assert!(
            path.is_some(),
        );
        let path_vec = path.unwrap();

        assert_eq!(path_vec.first().unwrap(), &Vector2::new(0, 3));
        assert_eq!(path_vec.last().unwrap(), &Vector2::new(5, 3));
    }

    #[test]
    fn test_maze_3() {
        let wall_map = vec![
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
        ];
        let world = World::new(wall_map, vec![], vec![]);

        let path = find_path(4, 4, 0, 0, &world);

        assert!(
            path.is_some(),
        );
        let path_vec = path.unwrap();

        assert_eq!(path_vec.first().unwrap(), &Vector2::new(0, 0));
        assert_eq!(path_vec.last().unwrap(), &Vector2::new(4, 4));
    }
}
