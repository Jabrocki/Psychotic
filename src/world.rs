use std::collections::HashMap;

use sdl2::pixels::Color;

pub struct World {
    map: [[i8; 256]; 256],
    reg: HashMap<i8, Tile>,
}

fn register_tiles(reg: &mut HashMap<i8, Tile>) {
    reg.insert(0, Tile::new(Color::RGB(0, 0, 0))); // AIR
    reg.insert(1, Tile::new(Color::RGB(255, 255, 255))); // WALL
}

impl World {
    pub fn new() -> World {
        let mut reg: HashMap<i8, Tile> = HashMap::new();

        register_tiles(&mut reg);

        World {
            map: [[0; 256]; 256],
            reg: reg,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> i8 {
        self.map[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: i8) {
        self.map[y][x] = tile;
    }

    pub fn width(&self) -> usize {
        256
    }

    pub fn height(&self) -> usize {
        256
    }

    pub fn reg(&self) -> &HashMap<i8, Tile> {
        &self.reg
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    color: Color,
}

impl Tile {
    fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
