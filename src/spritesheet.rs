use ggez::graphics::{
    self,
    spritebatch::{SpriteBatch, SpriteIdx},
};
use ggez::nalgebra as na;

#[derive(Hash, Eq, PartialEq)]
pub struct Tile {
    pub sprite_layer: i32,
    pub sprite_id: i32,
}

#[derive(Clone)]
pub struct TileMap {
    pub sprite_dimensions: u32,
    pub num_tiles_x: u32,
    pub num_tiles_y: u32,
    pub image: graphics::Image,
}

impl TileMap {
    pub fn new(image: graphics::Image, sprite_dimensions: u32) -> Self {
        TileMap {
            sprite_dimensions,
            num_tiles_x: image.width() as u32 / sprite_dimensions,
            num_tiles_y: image.height() as u32 / sprite_dimensions,
            image,
        }
    }
}

pub struct SpriteLayer {
    tilemap: TileMap,
    pub batch: SpriteBatch,
}

impl SpriteLayer {
    pub fn new(tilemap: TileMap) -> Self {
        let image = tilemap.image.clone();

        SpriteLayer {
            tilemap,
            batch: SpriteBatch::new(image),
        }
    }

    pub fn add(&mut self, tile: &Tile, x: f32, y: f32) -> SpriteIdx {
        let x = x * self.tilemap.sprite_dimensions as f32;
        let y = y * self.tilemap.sprite_dimensions as f32;

        let sprite_x = tile.sprite_id as usize % self.tilemap.num_tiles_x as usize;
        let sprite_y = tile.sprite_id as usize / self.tilemap.num_tiles_y as usize;

        let draw_param = graphics::DrawParam::default()
            .src(graphics::Rect::new(
                (1.0 / self.tilemap.num_tiles_x as f32) * sprite_x as f32,
                (1.0 / self.tilemap.num_tiles_y as f32) * sprite_y as f32,
                1.0 / self.tilemap.num_tiles_x as f32,
                1.0 / self.tilemap.num_tiles_y as f32,
            ))
            .dest(na::Point2::new(x, y));

        self.batch.add(draw_param)
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}
