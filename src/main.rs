//! Game setup and very basic main loop.
//! All the actual work gets done in the Scene.

use std::env;
use std::path;

use ggez::nalgebra as na;
use ggez::{self, *};

mod game;
mod input;
mod resources;
mod scenes;
mod spritesheet;
mod types;
mod util;
mod world;

struct MainState {
    scenes: scenes::Stack,
    input_binding: input::Binding,
}

impl MainState {
    fn new(ctx: &mut Context, resource_path: &path::Path) -> Self {
        let world = world::World::new(resource_path);
        let mut scenestack = scenes::Stack::new(ctx, world);
        let initial_scene = Box::new(scenes::menu::MenuScene::new(ctx, &mut scenestack.world));
        let level_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        scenestack.push(level_scene);
        scenestack.push(initial_scene);

        Self {
            input_binding: input::create_input_binding(),
            scenes: scenestack,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update(ctx);
        }
        self.scenes.world.resources.sync(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let screen = graphics::screen_coordinates(&ctx);

        graphics::clear(ctx, graphics::Color::from((0.0, 0.0, 0.4, 0.0)));
        self.scenes.draw(ctx);

        let fps = timer::fps(ctx);
        let mut fps_display = graphics::Text::new(format!("FPS: {}", fps));

        fps_display.set_bounds(na::Point2::new(400.0, 1000.0), graphics::Align::Left);

        graphics::draw(
            ctx,
            &fps_display,
            (na::Point2::new(0.0, screen.y), graphics::WHITE),
        )?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.world.input.update_effect(ev, true);
            self.scenes.input(input::InputEvent::InputEffect(ev), false);
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.world.input.update_effect(ev, false);
            self.scenes.input(input::InputEvent::InputEffect(ev), false);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        let ev = input::MouseEvent {
            x: x,
            y: y,
            dx: dx,
            dy: dy,
        };

        self.scenes.input(input::InputEvent::MouseEffect(ev), false);
    }
}

fn main() {
    util::setup_logging();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    println!("Resource dir: {:?}", resource_dir);

    let cb = ContextBuilder::new("Tower", "Tower")
        .window_setup(conf::WindowSetup::default().title("Tower"))
        .window_mode(conf::WindowMode::default())
        .add_resource_path(&resource_dir);

    let (ctx, ev) = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx, &resource_dir);

    ggez::graphics::set_default_filter(ctx, ggez::graphics::FilterMode::Nearest);

    if let Err(e) = event::run(ctx, ev, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
