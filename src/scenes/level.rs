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
    kiwi: warmy::Res<resources::Image>,
    bg: warmy::Res<resources::Image>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let kiwi = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/kiwi.png"), ctx)
            .unwrap();

        let bg = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/cloudy.png"), ctx)
            .unwrap();

        let board = world.boards.get_mut(0).unwrap();

        board.tiles.push(Unit::new());
        board.tiles.push(Unit::new());

        LevelScene { done, kiwi, bg }
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
                println!("x: {}, y: {}", position.x, position.y);

                graphics::draw(
                    ctx,
                    &(self.kiwi.borrow().0),
                    graphics::DrawParam::default().dest(na::Point2::new(
                        (position.x * self.kiwi.borrow().0.width() as u32) as f32,
                        position.y as f32,
                    )),
                )?;
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, _ev: input::Event, _started: bool) {
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }
}
