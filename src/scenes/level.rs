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

const SCALE_X: f32 = 1.5;
const SCALE_Y: f32 = 1.5;

#[derive(PartialEq)]
pub enum LevelState {
    PickUnit,
    RoundStart,
    RoundEnd,
}

pub enum UserAction {
    BuildUnit,
}

pub struct LevelScene {
    done: bool,
    sprite_layer: SpriteLayer,
    bg: warmy::Res<resources::Image>,
    island: warmy::Res<resources::Image>,
    chicken_definition: warmy::Res<resources::MobDefinition>,
    state: LevelState,
    placed_units: u32,
    spawned_mobs: u32,
    current_ticks: u32,
    current_user_action: Option<UserAction>,
    paths: Vec<na::Point2<i32>>,
    hovered_tile: Option<na::Point2<u32>>,
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
            hovered_tile: None,
            current_ticks: 0,
            placed_units: 0,
            spawned_mobs: 0,
            current_user_action: None,
            state: LevelState::PickUnit,
            sprite_layer: SpriteLayer::new(tilemap),
        }
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        let dt = 1.0 / 60.0;

        if self.state == LevelState::RoundStart {
            self.current_ticks += 1;

            if self.spawned_mobs < 10 {
                if self.current_ticks % 60 == 0 {
                    for board in &mut gameworld.boards {
                        let chicken: mob::MobEntity = (&self.chicken_definition.borrow().0).into();
                        board.mobs.push(chicken);
                    }

                    self.spawned_mobs += 1;
                }
            }
        }

        if self.spawned_mobs == 10
            && gameworld.boards.iter().all(|board| board.mobs.len() == 0)
            && self.state == LevelState::RoundStart
        {
            self.state = LevelState::RoundEnd;
        }

        for board in &mut gameworld.boards {
            for mob in board.mobs.iter_mut() {
                mob.update(dt);

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

        if let Some(action) = &self.current_user_action {
            match action {
                UserAction::BuildUnit => {
                    if self.placed_units < 5 && self.state == LevelState::PickUnit {
                        if let Some(hovered_tile) = self.hovered_tile {
                            let board = gameworld.boards.get_mut(0).unwrap();

                            board.tiles.push(unit::Unit {
                                unit_type: unit::UnitType::Warrior,
                                range: 36.0,
                                tile_position: na::Point2::new(
                                    hovered_tile.x as i32,
                                    hovered_tile.y as i32,
                                ),
                                ..unit::Unit::default()
                            });

                            self.placed_units += 1;

                            if self.placed_units == 5 {
                                self.state = LevelState::RoundStart;
                                self.current_ticks = 0;
                            }
                        }
                    }
                }
            }
        }

        self.current_user_action = None;

        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let dt =
            (ggez::timer::remaining_update_time(ctx).as_millis() as f32 / 1000.0) / (1.0 / 60.0);

        graphics::draw(
            ctx,
            &(self.bg.borrow().0),
            graphics::DrawParam::default().scale(na::Vector2::new(4.0, 4.0)),
        )?;

        let island_dimension = graphics::Rect::new(0.0, 0.0, 656.0 * SCALE_X, 656.0 * SCALE_Y);
        let calculated_dimensions = gameworld.screen.center_fit(&island_dimension);

        graphics::draw(
            ctx,
            &(self.island.borrow().0),
            graphics::DrawParam::default()
                .scale(na::Vector2::new(SCALE_X, SCALE_Y))
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
                    (unit.tile_position.x as f32 * 16.0) + 8.0,
                    (unit.tile_position.y as f32 * 16.0) + 8.0,
                );

                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::stroke(2.0),
                    na::Point2::new(
                        (unit.tile_position.x as f32 * 16.0) + (unit.range / 2.0) + 6.0,
                        (unit.tile_position.y as f32 * 16.0) + (unit.range / 2.0) + 6.0,
                    ),
                    unit.range,
                    1.0,
                    graphics::Color::new(1.0, 0.0, 0.0, 0.35),
                )?;

                if let Some(hover_coords) = self.hovered_tile {
                    if hover_coords.x == unit.tile_position.x as u32
                        && hover_coords.y == unit.tile_position.y as u32
                    {
                        let mut type_display =
                            graphics::Text::new(format!("Type {:?}", unit.unit_type));
                        type_display
                            .set_bounds(na::Point2::new(200.0, 50.0), graphics::Align::Left);
                        graphics::draw(
                            ctx,
                            &type_display,
                            graphics::DrawParam::default().dest(na::Point2::new(
                                ((unit.tile_position.x as f32 * 16.0) + 4.0) * SCALE_X
                                    + calculated_dimensions.x,
                                ((unit.tile_position.y as f32 * 16.0) - 4.0) * SCALE_Y
                                    + calculated_dimensions.y,
                            )),
                        )?;
                    }
                }

                graphics::draw(
                    ctx,
                    &circle,
                    graphics::DrawParam::default()
                        .dest(na::Point2::new(
                            calculated_dimensions.x,
                            calculated_dimensions.y,
                        ))
                        .scale(na::Vector2::new(SCALE_X, SCALE_Y)),
                )?;
            }

            for mob in board.mobs.iter() {
                self.sprite_layer.add(
                    &Tile {
                        sprite_layer: 0,
                        sprite_id: 350,
                    },
                    mob.last_position.x + (mob.position.x - mob.last_position.x) * dt,
                    mob.last_position.y + (mob.position.y - mob.last_position.y) * dt,
                );

                if mob.show_health_bar() {
                    let health_percentage = mob.current_health as f32 / mob.max_health as f32;
                    let width = 20.0 * health_percentage;

                    let current_bar = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(mob.position.x + 4.0, mob.position.y + 4.0, width, 2.0),
                        graphics::Color::new(0.0, 1.0, 0.0, 1.0),
                    )?;

                    let full_bar = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(mob.position.x + 4.0, mob.position.y + 4.0, 20.0, 2.0),
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
                            .scale(na::Vector2::new(SCALE_X, SCALE_Y)),
                    )?;

                    graphics::draw(
                        ctx,
                        &current_bar,
                        graphics::DrawParam::default()
                            .dest(na::Point2::new(
                                calculated_dimensions.x,
                                calculated_dimensions.y,
                            ))
                            .scale(na::Vector2::new(SCALE_X, SCALE_Y)),
                    )?;
                }
            }
        }

        // Given a 15x15 board and the screen dimensions,
        // Rect = screen.get_center_for_rect(Rect{ x: 0.0, y: 0.0, width: BOARD_WIDTH, height: BOARD_HEIGHT})
        // each tile is 16px * 40 = 640px unscaled width & height

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

        if let Some(hover_coords) = self.hovered_tile {
            let tile_hover = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    (hover_coords.x as f32 * 16.0) + 8.0,
                    (hover_coords.y as f32 * 16.0) + 8.0,
                    16.0,
                    16.0,
                ),
                graphics::Color::new(0.0, 1.0, 0.0, 1.0),
            )?;

            graphics::draw(
                ctx,
                &tile_hover,
                graphics::DrawParam::default()
                    .dest(na::Point2::new(
                        calculated_dimensions.x,
                        calculated_dimensions.y,
                    ))
                    .scale(na::Vector2::new(1.5, 1.5)),
            )?;
        }

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

            if gameworld.input.get_button_pressed(input::Button::Select) {
                self.current_user_action = Some(UserAction::BuildUnit);
            }
        }

        if let input::InputEvent::MouseEffect(effect) = ev {
            // @TODO(vy): Clean this up, but this calculates the mouse position relative to the
            // sprite tileboard created at scale.
            let (x, y) = (effect.x, effect.y);
            let island_dimension = graphics::Rect::new(0.0, 0.0, 656.0 * SCALE_X, 656.0 * SCALE_Y);
            let calculated_dimensions = gameworld.screen.center_fit(&island_dimension);

            let offset_x = ((x - calculated_dimensions.x) - 24.0) / SCALE_X;
            let offset_y = ((y - calculated_dimensions.y) - 24.0) / SCALE_Y;

            self.hovered_tile = Some(na::Point2::new(
                (na::clamp(offset_x, 0.0, 656.0 * SCALE_X) / 16.0) as u32,
                (na::clamp(offset_y, 0.0, 656.0 * SCALE_Y) / 16.0) as u32,
            ));
        }
    }
}
