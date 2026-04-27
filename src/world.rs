pub struct World {
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}

impl World {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        let height = map.len();
        let width = if height > 0 { map[0].len() } else { 0 };
        Self { width, height, map }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        if y < self.height && x < self.width {
            self.map[y][x]
        } else {
            1
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tex_id: u8) {
        self.map[y][x] = tex_id;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
