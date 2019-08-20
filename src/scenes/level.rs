use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez_goodies::scene;
use warmy;

use crate::game::unit::Unit;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::world::World;

pub struct LevelScene {
    done: bool,
    spritebatch: graphics::spritebatch::SpriteBatch,
    bg: warmy::Res<resources::Image>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;

        let bg = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/cloudy.png"), ctx)
            .unwrap();

        let spritesheet = graphics::Image::new(ctx, "/images/overworld_tileset_grass.png").unwrap();

        let board = world.boards.get_mut(0).unwrap();

        board.tiles.push(Unit::new());
        board.tiles.push(Unit::new());

        LevelScene {
            done,
            spritebatch: graphics::spritebatch::SpriteBatch::new(spritesheet),
            bg,
        }
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, _gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
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
                let p = graphics::DrawParam::default()
                    .dest(na::Point2::new(
                        (position.x as u32) as f32 * 2.0 / 12.0,
                        position.y as f32,
                    ))
                    .src(graphics::Rect::new(
                        2.0 * (1.0 / 12.0),
                        5.0 * (1.0 / 21.0),
                        1.0 / 12.0,
                        1.0 / 21.0,
                    ))
                    .scale(na::Vector2::new(2.0, 2.0));

                self.spritebatch.add(p);
            }
        }

        graphics::draw(ctx, &self.spritebatch, graphics::DrawParam::default())?;

        self.spritebatch.clear();

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
