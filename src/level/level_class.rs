use std::sync::Arc;

use crate::level::{
    generate_edge_list::generate_edge_list, kruskarl::kruskarl, room::Room, room_file::RoomFile,
};
use rand::RngExt;

// Helper structk instead of tuple - makes it more readable
#[derive(Clone)]
pub struct RoomData {
    pub room_file: RoomFile,
    pub count: usize,
}

static MAX_ROOM_ITER: i32 = 1000;

// TODO: mayde this private
pub struct Level {
    width: usize,
    height: usize,
    pub wall_map: Vec<Vec<i16>>,
    pub ceil_map: Vec<Vec<i16>>,
    pub floor_map: Vec<Vec<i16>>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            wall_map: vec![vec![-1; width]; height],
            ceil_map: vec![vec![-1; width]; height],
            floor_map: vec![vec![-1; width]; height],
        }
    }

    pub fn generate(&mut self, room_data: Vec<RoomData>) {
        let room_list = self.place_rooms(room_data);
        self.connect_rooms(room_list);
    }

    // Pub function for unit tests - do not use it in the main code
    pub fn place_rooms(&mut self, mut room_data: Vec<RoomData>) -> Vec<Room> {
        // Function to place rooms in the level
        let mut rooms = Vec::new();
        while let Some(room_record) = room_data.pop() {
            self.place_single_room_record(room_record, &mut rooms);
        }
        rooms
    }

    fn place_single_room_record(&mut self, room_record: RoomData, rooms: &mut Vec<Room>) {
        // Function to place a single room in the level
        let mut rng = rand::rng();
        let room_file = Arc::new(room_record.room_file);
        for _ in 0..room_record.count {
            let mut room_placed = false;
            for _ in 0..MAX_ROOM_ITER {
                let x = rng.random_range(0..self.width - room_file.get_width());
                let y = rng.random_range(0..self.height - room_file.get_height());
                // Check if the room can be placed at (x, y)
                if self.can_place_room(&room_file, x, y) {
                    self.place_room_on_maps(&room_file, x, y);
                    let room = Room::new(Arc::clone(&room_file), nalgebra::Vector2::new(x, y));
                    rooms.push(room);
                    room_placed = true;
                    break;
                }
            }

            if !room_placed {
                panic!("Can't be placed!!")
            }
        }
    }

    fn can_place_room(&self, room: &RoomFile, x: usize, y: usize) -> bool {
        let height = room.get_height() as isize;
        let width = room.get_width() as isize;
        let y = y as isize;
        let x = x as isize;
        for i in -1..=height {
            for j in -1..=width {
                let ny = y + i;
                let nx = x + j;
                if ny < 0
                    || nx < 0
                    || ny >= self.wall_map.len() as isize
                    || nx >= self.wall_map[0].len() as isize
                {
                    return false;
                }
                if self.wall_map[ny as usize][nx as usize] != -1 {
                    return false; // The space is not empty
                }
            }
        }
        true
    }

    fn place_room_on_maps(&mut self, room: &RoomFile, x: usize, y: usize) {
        // Function to place a room on the wall, ceil and floor maps
        for i in 0..room.get_height() {
            for j in 0..room.get_width() {
                self.wall_map[y + i][x + j] = room.get_wall_map()[i][j];
                self.ceil_map[y + i][x + j] = room.get_ceil_map()[i][j];
                self.floor_map[y + i][x + j] = room.get_floor_map()[i][j];
            }
        }
    }

    // Last step was split into files due
    fn connect_rooms(&mut self, room_list: Vec<Room>) {
        let edges = generate_edge_list(room_list);
        let tree_edges = kruskarl(edges);
        self.connect_corridors(&tree_edges);
        self.clean_up_maps();
    }

    // Getters
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_wall_map(&self) -> &Vec<Vec<i16>> {
        &self.wall_map
    }

    pub fn get_wall_tile(&self, x: usize, y: usize) -> i16 {
        self.wall_map[y][x]
    }

    pub fn get_ceil_map(&self) -> &Vec<Vec<i16>> {
        &self.ceil_map
    }

    pub fn get_floor_map(&self) -> &Vec<Vec<i16>> {
        &self.floor_map
    }

    // Setters
    pub fn set_wall_map_tile(&mut self, x: usize, y: usize, value: i16) {
        self.wall_map[y][x] = value;
    }

    pub fn set_ceil_map_tile(&mut self, x: usize, y: usize, value: i16) {
        self.ceil_map[y][x] = value;
    }

    pub fn set_floor_map_tile(&mut self, x: usize, y: usize, value: i16) {
        self.floor_map[y][x] = value;
    }

    // Clean up maps -1 -> 1 and -2 -> 0
    fn clean_up_maps(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                // Clean up wall map
                if self.wall_map[y][x] == -1 {
                    self.wall_map[y][x] = 1;
                } else if self.wall_map[y][x] == -2 {
                    self.wall_map[y][x] = 0;
                }

                // Clean up ceil map
                if self.ceil_map[y][x] == -1 {
                    self.ceil_map[y][x] = 1;
                }

                // Clean up floor map
                if self.floor_map[y][x] == -1 {
                    self.floor_map[y][x] = 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_single_room_record() {
        let room = RoomFile::from_file("rooms/small_room.json");
        let room_record = RoomData {
            room_file: room,
            count: 2,
        };

        let result = std::panic::catch_unwind(|| {
            let mut level = Level::new(16, 16);
            let mut room_list = Vec::new();
            level.place_single_room_record(room_record, &mut room_list);
            level
        });

        assert!(result.is_ok());

        let Some(level) = result.ok() else {
            panic!("Result is empty");
        };
        let width = level.get_width();
        let height = level.get_height();
        let mut count_doors = 0;
        let mut count_walls = 0;
        let mut count_air = 0;

        for x in 0..width {
            for y in 0..height {
                let tile = level.get_wall_tile(x, y);
                if tile == -2 {
                    count_doors += 1;
                }

                if tile > 0 {
                    count_walls += 1;
                }

                if tile == 0 {
                    count_air += 1;
                }
            }
        }

        assert!(count_doors == 8);
        assert!(count_walls == 8);
        assert!(count_air == 2);
    }

    #[test]
    fn test_place_rooms() {
        let small_room = RoomFile::from_file("rooms/small_room.json");
        let small_room_walls = small_room
            .get_wall_map()
            .iter()
            .flatten()
            .filter(|&&tile| tile > 0)
            .count();
        let small_room_doors = small_room
            .get_wall_map()
            .iter()
            .flatten()
            .filter(|&&tile| tile == -2)
            .count();
        let small_room_air = small_room
            .get_wall_map()
            .iter()
            .flatten()
            .filter(|&&tile| tile == 0)
            .count();

        let medium_room = RoomFile::from_file("rooms/medium_room.json");

        let medium_room_walls = medium_room
            .get_wall_map()
            .iter()
            .flatten()
            .filter(|&&tile| tile > 0)
            .count();

        let medium_room_doors = medium_room
            .get_wall_map()
            .iter()
            .flatten()
            .filter(|&&tile| tile == -2)
            .count();

        let medium_room_air = medium_room
            .get_wall_map()
            .iter()
            .flatten()
            .filter(|&&tile| tile == 0)
            .count();

        let room_data = vec![
            RoomData {
                room_file: small_room,
                count: 2,
            },
            RoomData {
                room_file: medium_room,
                count: 2,
            },
        ];

        let result = std::panic::catch_unwind(|| {
            let mut level = Level::new(100, 100);
            level.place_rooms(room_data);
            level
        });

        assert!(result.is_ok());

        let Some(level) = result.ok() else {
            panic!("Result is empty");
        };

        let width = level.get_width();
        let height = level.get_height();
        let mut count_doors = 0;
        let mut count_walls = 0;
        let mut count_air = 0;

        for x in 0..width {
            for y in 0..height {
                let tile = level.get_wall_tile(x, y);

                if tile == -2 {
                    count_doors += 1;
                }

                if tile > 0 {
                    count_walls += 1;
                }

                if tile == 0 {
                    count_air += 1;
                }
            }
        }

        assert!(count_doors == 2 * (small_room_doors + medium_room_doors));
        assert!(count_walls == 2 * (small_room_walls + medium_room_walls));
        assert!(count_air == 2 * (small_room_air + medium_room_air));

        for walls in level.get_wall_map() {
            println!("{:?}", walls);
        }
    }
}
