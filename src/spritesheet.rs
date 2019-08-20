use ggez::graphics::{self, spritebatch::{SpriteIdx, SpriteBatch}};
use ggez::nalgebra as na;

use crate::resources;

#[derive(Hash, Eq, PartialEq)]
pub struct Tile {
    pub sprite_layer: i32,
    pub sprite_id: i32,
}

#[derive(Clone)]
pub struct TileMap {
    sprite_dimensions: u32,
    tile_width: u32,
    tile_height: u32,
    num_tiles_x: u32,
    num_tiles_y: u32,
    image: resources::Image,
}

impl TileMap {
    pub fn new(
        ctx: &ggez::Context,
        image: resources::Image,
        sprite_dimensions: u32,
    ) -> Self {
        let screen = graphics::screen_coordinates(ctx);
        let num_tiles_x: u32 = (screen.x.floor() as u32 / sprite_dimensions) + 1;
        let num_tiles_y: u32 = (screen.y.floor() as u32 / sprite_dimensions) + 1;

        TileMap {
            sprite_dimensions,
            tile_width: image.0.width() as u32 / sprite_dimensions,
            tile_height: image.0.height() as u32 / sprite_dimensions,
            num_tiles_x,
            num_tiles_y,
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
        let image = tilemap.image.0.clone();

        SpriteLayer {
            tilemap,
            batch: SpriteBatch::new(image),
        }
    }

    pub fn add(&mut self, tile: &Tile, x: i32, y: i32) -> SpriteIdx {
        let x: usize = x as usize * self.tilemap.sprite_dimensions as usize;
        let y: usize = y as usize * self.tilemap.sprite_dimensions as usize;

        let sprite_x = tile.sprite_id as usize % self.tilemap.tile_width as usize;
        let sprite_y = tile.sprite_id as usize / self.tilemap.tile_width as usize;

        let draw_param = graphics::DrawParam::default()
            .src(
                graphics::Rect::new(
                    (1.0 / self.tilemap.tile_width as f32) * sprite_x as f32,
                    (1.0 / self.tilemap.tile_height as f32) * sprite_y as f32,
                    1.0 / self.tilemap.tile_width as f32,
                    1.0 / self.tilemap.tile_height as f32
            ))
            .dest(na::Point2::new(0.0, 0.0));

        self.batch.add(draw_param)
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}