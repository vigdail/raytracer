use crate::color::Color;

pub struct TileConfig {
    pub width: u32,
    pub height: u32,
}

impl TileConfig {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Tile {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        let capacity = (width * height) as usize;
        Self {
            x,
            y,
            width,
            height,
            data: vec![Color::default(); capacity],
        }
    }
}

pub fn split_surface(width: u32, height: u32, tile_width: u32, tile_height: u32) -> Vec<Tile> {
    let tiles_col = width / tile_width;
    let tiles_row = height / tile_height;

    let mut tiles = Vec::with_capacity((tiles_col * tiles_row) as usize);
    for x in 0..tiles_col {
        for y in 0..tiles_row {
            let tile = Tile::new(x * tile_width, y * tile_height, tile_width, tile_height);
            tiles.push(tile);
        }
    }

    tiles
}