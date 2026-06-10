use nalgebra::Vector2;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct RoomFile {
    width: usize,
    height: usize,
    wall_map: Vec<Vec<i16>>,
    ceil_map: Vec<Vec<i16>>,
    floor_map: Vec<Vec<i16>>,
}

impl RoomFile {
    pub fn from_file(path: &str) -> Self {
        /*
        eg file format:
        {
            "width": 10,
            "height": 10,
            "wall_map": [[1,1,1,1,-2,1,1,1,1,1],
                         [1,0,0,0,0,0,0,0,0,1],
                         [1,0,4,4,0,0,3,3,0,1],
                         [1,0,4,4,0,0,3,3,0,1],
                         [-2,0,0,0,0,0,0,0,0,1],
                         [1,0,0,0,0,0,0,0,0,-2],
                         [1,0,2,2,0,0,4,4,0,1],
                         [1,0,2,2,0,0,4,4,0,1],
                         [1,0,0,0,0,0,0,0,0,1],
                         [1,1,1,1,1,-2,1,1,1,1]],
            "ceil_map": [[1,1,1,1,1,1,1,1,1,1],
                         [1,1,1,1,1,1,1,1,1,1],
                         [2,2,2,2,2,2,2,2,2,2],
                         [2,2,2,2,2,2,2,2,2,2],
                         [1,1,1,1,1,1,1,1,1,1],
                         [1,1,1,1,1,1,1,1,1,1],
                         [3,3,3,3,3,3,3,3,3,3],
                         [3,3,3,3,3,3,3,3,3,3],
                         [1,1,1,1,1,1,1,1,1,1],
                         [1,1,1,1,1,1,1,1,1,1]],
            "floor_map": [[1,1,1,1,1,1,1,1,1,1],
                          [2,2,2,2,2,2,2,2,2,2],
                          [3,3,3,3,3,3,3,3,3,3],
                          [4,4,4,4,4,4,4,4,4,4],
                          [5,5,5,5,5,5,5,5,5,5],
                          [5,5,5,5,5,5,5,5,5,5],
                          [4,4,4,4,4,4,4,4,4,4],
                          [3,3,3,3,3,3,3,3,3,3],
                          [2,2,2,2,2,2,2,2,2,2],
                          [1,1,1,1,1,1,1,1,1,1]]
        }
        -2 : door
        0 : air
        postive numbers: texture id
        */
        let file_content = std::fs::read_to_string(path).expect("Failed to read room file");
        let room: RoomFile =
            serde_json::from_str(&file_content).expect("Failed to parse room file");
        if !room.validate_maps() {
            panic!("Invalid room maps");
        }
        room
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_door_positions(&self) -> Vec<Vector2<usize>> {
        let mut door_positions = Vec::new();
        for (y, row) in self.wall_map.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == -2 {
                    door_positions.push(Vector2::new(x, y));
                }
            }
        }
        door_positions
    }

    pub fn get_wall_map(&self) -> &Vec<Vec<i16>> {
        &self.wall_map
    }

    pub fn get_ceil_map(&self) -> &Vec<Vec<i16>> {
        &self.ceil_map
    }

    pub fn get_floor_map(&self) -> &Vec<Vec<i16>> {
        &self.floor_map
    }

    fn validate_maps(&self) -> bool {
        self.validate_maps_dimensions() && self.validate_wall_map()
    }

    fn validate_maps_dimensions(&self) -> bool {
        self.wall_map.len() == self.height
            && self.ceil_map.len() == self.height
            && self.floor_map.len() == self.height
            && self.wall_map.iter().all(|row| row.len() == self.width)
            && self.ceil_map.iter().all(|row| row.len() == self.width)
            && self.floor_map.iter().all(|row| row.len() == self.width)
    }

    fn validate_wall_map(&self) -> bool {
        for row in &self.wall_map {
            for &tile in row {
                if tile < -2 || tile == -1 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_room_from_file() {
        let room = RoomFile::from_file("rooms/test_room.json");
        assert_eq!(room.get_width(), 10);
        assert_eq!(room.get_height(), 10);
    }

    #[test]
    fn test_invalid_room_file() {
        let result = std::panic::catch_unwind(|| {
            RoomFile::from_file("rooms/invalid_room.json");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_get_door_positions() {
        let room = RoomFile::from_file("rooms/test_room.json");
        let door_positions = room.get_door_positions();
        assert_eq!(door_positions.len(), 4);
        assert!(door_positions.contains(&Vector2::new(4, 0)));
        assert!(door_positions.contains(&Vector2::new(0, 4)));
        assert!(door_positions.contains(&Vector2::new(9, 5)));
        assert!(door_positions.contains(&Vector2::new(5, 9)));
    }
}
