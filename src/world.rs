pub struct World {
    map: [[u8; 256]; 256],
}

impl World {
    pub fn new() -> World {
        World {
            map: [[0; 256]; 256],
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        self.map[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tex_id: u8) {
        self.map[y][x] = tex_id;
    }

    pub fn width(&self) -> usize {
        256
    }

    pub fn height(&self) -> usize {
        256
    }
}
