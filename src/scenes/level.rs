use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez_goodies::scene;
use warmy;

use crate::game::mob;
use crate::game::unit::Unit;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::spritesheet::{SpriteLayer, Tile, TileMap};
use crate::world::World;

pub struct LevelScene {
    done: bool,
    sprite_layer: SpriteLayer,
    bg: warmy::Res<resources::Image>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;

        let bg = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/cloudy.png"), ctx)
            .unwrap();

        let mut spritesheet =
            graphics::Image::new(ctx, "/images/overworld_tileset_grass.png").unwrap();

        /*
                let warrior_unit = world
                    .resources
                    .get::<resources::Unit>(&resources::Key::from_path("/units/basic/warrior.ron"), ctx);
        */

        spritesheet.set_filter(graphics::FilterMode::Nearest);

        let board = world.boards.get_mut(0).unwrap();

        for _ in 0..(15 * 15) {
            board.tiles.push(Unit::new());
        }

        let tilemap = TileMap::new(spritesheet, 16);

        let chicken_definition = world
            .resources
            .get::<resources::MobDefinition>(&resources::Key::from_path("/mobs/chicken.ron"), ctx)
            .unwrap();

        let chicken: mob::MobEntity = chicken_definition.borrow().0.into();

        board.mobs.push(chicken);

        LevelScene {
            done,
            bg,
            sprite_layer: SpriteLayer::new(tilemap),
        }
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        for board in &mut gameworld.boards {
            for mob in board.mobs.iter_mut() {
                mob.position = mob.position + mob.velocity;
            }
        }

        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::draw(
            ctx,
            &(self.bg.borrow().0),
            graphics::DrawParam::default().scale(na::Vector2::new(2.0, 2.0)),
        )?;

        for board in &gameworld.boards {
            for (position, _tile) in board.with_positions() {
                self.sprite_layer.add(
                    &Tile {
                        sprite_layer: 0,
                        sprite_id: 5,
                    },
                    position.x as f32,
                    position.y as f32,
                );
            }

            for mob in board.mobs.iter() {
                self.sprite_layer.add(
                    &Tile {
                        sprite_layer: 0,
                        sprite_id: 51,
                    },
                    mob.position.x,
                    mob.position.y,
                );
            }
        }

        // Given a 15x15 board and the screen dimensions,
        // Rect = screen.get_center_for_rect(Rect{ x: 0.0, y: 0.0, width: BOARD_WIDTH, height: BOARD_HEIGHT})
        // each tile is 16px * 15 = 240px unscaled width & height

        const SCALE_X: f32 = 2.0;
        const SCALE_Y: f32 = 2.0;
        let board_dimensions = graphics::Rect::new(0.0, 0.0, 240.0 * SCALE_X, 240.0 * SCALE_Y);
        let calculated_dimensions = gameworld.screen.center_fit(&board_dimensions);

        graphics::draw(
            ctx,
            &self.sprite_layer.batch,
            graphics::DrawParam::default()
                .dest(na::Point2::new(
                    calculated_dimensions.x,
                    calculated_dimensions.y,
                ))
                .scale(na::Vector2::new(SCALE_X, SCALE_Y)),
        )?;

        self.sprite_layer.clear();

        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        if let input::InputEvent::InputEffect(_) = ev {
            if gameworld.input.get_button_pressed(input::Button::Menu) {
                self.done = true;
            }
        }
    }
}
