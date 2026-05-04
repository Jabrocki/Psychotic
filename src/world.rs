pub struct World {
    width: usize,
    height: usize,
    wall_map: Vec<Vec<u8>>,
    ceil_map: Vec<Vec<u8>>,
    floor_map: Vec<Vec<u8>>,
}

impl World {
    pub fn new(wall_map: Vec<Vec<u8>>, ceil_map: Vec<Vec<u8>>, floor_map: Vec<Vec<u8>>) -> Self {
        let height = wall_map.len();
        let width = if height > 0 { wall_map[0].len() } else { 0 };
        Self {
            width,
            height,
            wall_map,
            ceil_map,
            floor_map,
        }
    }

    pub fn get_wall_tile(&self, x: usize, y: usize) -> u8 {
        if y < self.height && x < self.width {
            self.wall_map[y][x]
        } else {
            1
        }
    }

    pub fn get_ceil_tile(&self, x: usize, y: usize) -> u8 {
        if y < self.height && x < self.width {
            self.ceil_map[y][x]
        } else {
            0
        }
    }

    pub fn get_floor_tile(&self, x: usize, y: usize) -> u8 {
        if y < self.height && x < self.width {
            self.floor_map[y][x]
        } else {
            0
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
