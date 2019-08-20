use ggez;
use ggez_goodies::scene;

use crate::input;
use crate::scenes;
use crate::world::World;

pub struct MenuScene {
    done: bool,
}

impl MenuScene {
    pub fn new(_ctx: &mut ggez::Context, _world: &mut World) -> Self {
        let done = false;

        MenuScene { done }
    }
}

impl scene::Scene<World, input::Event> for MenuScene {
    fn update(&mut self, _gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut World, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "MenuScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        if let input::InputEvent::InputEffect(_) = ev {
            if gameworld.input.get_button_pressed(input::Button::Menu) {
                self.done = true;
            }
        }
    }
}
