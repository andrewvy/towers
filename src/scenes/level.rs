use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez_goodies::scene;
use warmy;

use crate::game::mob;
use crate::game::unit;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::spritesheet::{SpriteLayer, Tile, TileMap};
use crate::world::World;

pub struct LevelScene {
    done: bool,
    sprite_layer: SpriteLayer,
    bg: warmy::Res<resources::Image>,
    island: warmy::Res<resources::Image>,
    chicken_definition: warmy::Res<resources::MobDefinition>,
    paths: Vec<na::Point2<i32>>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;

        let bg = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/bg.png"), ctx)
            .unwrap();

        let island = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/island.png"), ctx)
            .unwrap();

        let mut spritesheet =
            graphics::Image::new(ctx, "/images/overworld_tileset_grass.png").unwrap();

        spritesheet.set_filter(graphics::FilterMode::Nearest);

        let board = world.boards.get_mut(0).unwrap();

        board.tiles.push(unit::Unit {
            unit_type: unit::UnitType::Warrior,
            range: 36.0,
            tile_position: na::Point2::new(5, 10),
            ..unit::Unit::default()
        });

        board.tiles.push(unit::Unit {
            unit_type: unit::UnitType::Warrior,
            range: 36.0,
            tile_position: na::Point2::new(5, 19),
            ..unit::Unit::default()
        });

        let tilemap = TileMap::new(spritesheet, 16);

        let chicken_definition = world
            .resources
            .get::<resources::MobDefinition>(&resources::Key::from_path("/mobs/chicken.ron"), ctx)
            .unwrap();

        let paths = board.calculate_paths().unwrap();

        LevelScene {
            done,
            bg,
            island,
            paths,
            chicken_definition,
            sprite_layer: SpriteLayer::new(tilemap),
        }
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        let ticks = ggez::timer::ticks(ctx) % 60;

        if ticks == 0 {
            for board in &mut gameworld.boards {
                let chicken: mob::MobEntity = (&self.chicken_definition.borrow().0).into();
                board.mobs.push(chicken);
            }
        }

        for board in &mut gameworld.boards {
            for mob in board.mobs.iter_mut() {
                mob.update();

                if mob.status == mob::MobEntityStatus::FinishedPath {
                    mob.path_index += 1;

                    if (mob.path_index as usize) < self.paths.len() {
                        let new_path = self.paths[mob.path_index as usize];

                        mob.destination = na::Point2::new(new_path.x as f32, new_path.y as f32);
                        mob.status = mob::MobEntityStatus::Walking;
                    }
                }
            }

            board.update();
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
            graphics::DrawParam::default().scale(na::Vector2::new(4.0, 4.0)),
        )?;

        let island_dimension = graphics::Rect::new(0.0, 0.0, 656.0 * 1.5, 656.0 * 1.5);
        let calculated_dimensions = gameworld.screen.center_fit(&island_dimension);

        graphics::draw(
            ctx,
            &(self.island.borrow().0),
            graphics::DrawParam::default()
                .scale(na::Vector2::new(1.5, 1.5))
                .dest(na::Point2::new(
                    calculated_dimensions.x,
                    calculated_dimensions.y,
                )),
        )?;

        for board in &gameworld.boards {
            for unit in &board.tiles {
                self.sprite_layer.add(
                    &Tile {
                        sprite_layer: 0,
                        sprite_id: 5,
                    },
                    (unit.tile_position.x as f32 * 16.0) + 4.0,
                    (unit.tile_position.y as f32 * 16.0) + 4.0,
                );

                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::stroke(2.0),
                    na::Point2::new(
                        (unit.tile_position.x as f32 * 16.0) + (unit.range / 2.0) + 4.0,
                        (unit.tile_position.y as f32 * 16.0) + (unit.range / 2.0) + 4.0,
                    ),
                    unit.range,
                    1.0,
                    graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                )?;

                graphics::draw(
                    ctx,
                    &circle,
                    graphics::DrawParam::default()
                        .dest(na::Point2::new(
                            calculated_dimensions.x,
                            calculated_dimensions.y,
                        ))
                        .scale(na::Vector2::new(1.5, 1.5)),
                )?;
            }

            for mob in board.mobs.iter() {
                self.sprite_layer.add(
                    &Tile {
                        sprite_layer: 0,
                        sprite_id: 350,
                    },
                    mob.position.x,
                    mob.position.y,
                );

                if mob.show_health_bar() {
                    let health_percentage = mob.current_health as f32 / mob.max_health as f32;
                    let width = 20.0 * health_percentage;

                    let current_bar = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(mob.position.x, mob.position.y, width, 2.0),
                        graphics::Color::new(0.0, 1.0, 0.0, 1.0),
                    )?;

                    let full_bar = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(mob.position.x, mob.position.y, 20.0, 2.0),
                        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
                    )?;

                    graphics::draw(
                        ctx,
                        &full_bar,
                        graphics::DrawParam::default()
                            .dest(na::Point2::new(
                                calculated_dimensions.x,
                                calculated_dimensions.y,
                            ))
                            .scale(na::Vector2::new(1.5, 1.5)),
                    )?;

                    graphics::draw(
                        ctx,
                        &current_bar,
                        graphics::DrawParam::default()
                            .dest(na::Point2::new(
                                calculated_dimensions.x,
                                calculated_dimensions.y,
                            ))
                            .scale(na::Vector2::new(1.5, 1.5)),
                    )?;
                }
            }
        }

        // Given a 15x15 board and the screen dimensions,
        // Rect = screen.get_center_for_rect(Rect{ x: 0.0, y: 0.0, width: BOARD_WIDTH, height: BOARD_HEIGHT})
        // each tile is 16px * 40 = 640px unscaled width & height

        const SCALE_X: f32 = 1.5;
        const SCALE_Y: f32 = 1.5;

        let board_dimensions = graphics::Rect::new(0.0, 0.0, 640.0 * SCALE_X, 640.0 * SCALE_Y);
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
