use crate::level::room::Room;
use nalgebra::Vector2;
use rand::RngExt;

// Helper structk instead of tuple - makes it more readable
pub struct RoomData {
    room: Room,
    count: usize,
}

static MAX_ROOM_ITER: i32 = 1000;

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
            self.place_single_room_record(room_record);
        }
    }

    fn place_single_room_record(&mut self, room_record: RoomData) {
        // Function to place a single room in the level
        let mut rng = rand::rng();
        for _ in 0..room_record.count {
            let mut room_placed = false;
            for _ in 0..MAX_ROOM_ITER {
                let x = rng.random_range(0..self.width - room_record.room.get_width());
                let y = rng.random_range(0..self.height - room_record.room.get_height());
                // Check if the room can be placed at (x, y)
                if self.can_place_room(&room_record.room, x, y) {
                    self.place_room_on_maps(&room_record.room, x, y);
                    room_placed = true;
                    break;
                }
            }

            if !room_placed {
                panic!("Can't be placed!!")
            }
        }
    }

    fn can_place_room(&self, room: &Room, x: usize, y: usize) -> bool {
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

    fn place_room_on_maps(&mut self, room: &Room, x: usize, y: usize) {
        // Function to place a room on the wall, ceil and floor maps
        for i in 0..room.get_height() {
            for j in 0..room.get_width() {
                self.wall_map[y + i][x + j] = room.get_wall_map()[i][j];
                self.ceil_map[y + i][x + j] = room.get_ceil_map()[i][j];
                self.floor_map[y + i][x + j] = room.get_floor_map()[i][j];
            }
        }
        self.rooms.push((room.clone(), Vector2::new(x, y)));
    }

    // Last step was split into files due
    fn connect_rooms(&mut self) {
        // Function to connect rooms with corridors
        let edges = self.generate_edge_list();
        let tree_edges = self.kruskarl(edges);
        self.connect_corridors(tree_edges);
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

    pub fn get_wall_tile(&self, x: isize, y: isize) -> i16 {
        if x < 0
            || y < 0
            || y as usize >= self.wall_map.len()
            || x as usize >= self.wall_map[0].len()
        {
            return -1; // Out of bounds
        }
        self.wall_map[y as usize][x as usize]
    }

    pub fn get_ceil_map(&self) -> &Vec<Vec<i16>> {
        &self.ceil_map
    }

    pub fn get_floor_map(&self) -> &Vec<Vec<i16>> {
        &self.floor_map
    }

    pub fn get_rooms(&self) -> &Vec<(Room, Vector2<usize>)> {
        &self.rooms
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_single_room_record() {
        let room = Room::from_file("rooms/small_room.json");
        let room_record = RoomData { room, count: 2 };

        let result = std::panic::catch_unwind(|| {
            let mut level = Level::new(16, 16);
            level.place_single_room_record(room_record);
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
                let tile = level.get_wall_tile(x as isize, y as isize);
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
        let small_room = Room::from_file("rooms/small_room.json");
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

        let medium_room = Room::from_file("rooms/medium_room.json");

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
                room: small_room,
                count: 2,
            },
            RoomData {
                room: medium_room,
                count: 2,
            },
        ];

        let result = std::panic::catch_unwind(|| {
            let mut level = Level::new(15, 15);
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
                let tile = level.get_wall_tile(x as isize, y as isize);

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
