use ggez::nalgebra as na;

use pathfinding::prelude::astar;

use crate::game::mob::MobEntity;
use crate::game::unit::Unit;

pub struct Board {
    pub tiles: Vec<Option<Unit>>,
    pub mobs: Vec<MobEntity>,

    waypoints: Vec<(na::Point2<i32>, na::Point2<i32>)>,
}

const BOARD_HEIGHT: usize = 40;
const BOARD_WIDTH: usize = 40;
const BOARD_SIZE: usize = BOARD_HEIGHT * BOARD_WIDTH;

struct DamageEvent {
    damage: u32,
    source: na::Point2<f32>,
    range: f32,
    applied: bool,
}

impl DamageEvent {
    pub fn new(damage: u32, source: na::Point2<f32>, range: f32) -> Self {
        Self {
            damage,
            source,
            range,
            applied: false,
        }
    }
}

impl Board {
    pub fn at_position<'a>(&'a self, coordinates: &na::Point2<i32>) -> Option<&Unit> {
        self.tiles[coordinates.x as usize * coordinates.y as usize].as_ref()
    }

    pub fn with_positions<'a>(&'a self) -> Vec<(na::Point2<u32>, Option<&Unit>)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(index, unit)| {
                (
                    na::Point2::new((index % BOARD_HEIGHT) as u32, (index / BOARD_WIDTH) as u32),
                    unit.as_ref(),
                )
            })
            .collect()
    }

    pub fn with_positions_mut<'a>(&'a mut self) -> Vec<(na::Point2<u32>, &mut Option<Unit>)> {
        self.tiles
            .iter_mut()
            .enumerate()
            .map(|(index, unit)| {
                (
                    na::Point2::new((index % BOARD_HEIGHT) as u32, (index / BOARD_WIDTH) as u32),
                    unit,
                )
            })
            .collect()
    }

    pub fn calculate_path(
        &self,
        from: &na::Point2<i32>,
        goal: &na::Point2<i32>,
    ) -> Option<(Vec<na::Point2<i32>>, i32)> {
        astar(
            from,
            |p| self.successors(p),
            |p| ((goal.x - p.x).abs() + (goal.y - p.y).abs()).abs(),
            |p| *p == *goal,
        )
    }

    pub fn calculate_paths(&self) -> Option<(Vec<na::Point2<i32>>)> {
        let results = self
            .waypoints
            .iter()
            .map(|(start, end)| self.calculate_path(start, end));

        if results.clone().any(|result| result.is_none()) {
            None
        } else {
            Some(results.fold(Vec::with_capacity(32), |mut acc, result| {
                let (mut points, _) = result.unwrap();
                acc.append(&mut points);
                acc
            }))
        }
    }

    pub fn update(&mut self) {
        let mut damage_events: Vec<DamageEvent> = self
            .with_positions_mut()
            .iter_mut()
            .filter_map(|(position, unit)| {
                if let Some(unit) = unit {
                    match unit.perform_attack() {
                        Some(damage) => {
                            let real_position =
                                na::Point2::new(position.x as f32 * 16.0, position.y as f32 * 16.0);

                            Some(DamageEvent::new(damage, real_position, unit.range))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();

        for damage_event in damage_events.iter_mut() {
            let mut index = 0;

            while index != self.mobs.len() {
                if damage_event.applied {
                    return;
                }

                let mob = &mut self.mobs[index];
                let distance = na::distance(&mob.position, &damage_event.source);
                if distance <= damage_event.range {
                    mob.damage(damage_event.damage);
                    damage_event.applied = true;

                    if mob.is_alive() {
                        index += 1;
                    } else {
                        self.mobs.remove(index);
                    }
                } else {
                    index += 1;
                }
            }
        }
    }

    // @TODO(vy): This should handle the cases of preventing movement through diagonals.
    fn successors(&self, point: &na::Point2<i32>) -> Vec<(na::Point2<i32>, i32)> {
        let mut tiles: Vec<na::Point2<i32>> = Vec::with_capacity(8);

        if point.x > 0 {
            tiles.push(na::Point2::new(point.x - 1, point.y));
            tiles.push(na::Point2::new(point.x - 1, point.y + 1));
        }

        if point.y > 0 {
            tiles.push(na::Point2::new(point.x + 1, point.y - 1));
            tiles.push(na::Point2::new(point.x, point.y - 1));
        }

        if point.x > 0 && point.y > 0 {
            tiles.push(na::Point2::new(point.x - 1, point.y - 1));
            tiles.push(na::Point2::new(point.x, point.y - 1));
        }

        tiles.push(na::Point2::new(point.x, point.y + 1));
        tiles.push(na::Point2::new(point.x + 1, point.y));
        tiles.push(na::Point2::new(point.x + 1, point.y + 1));

        tiles
            .into_iter()
            .filter(|pos| self.at_position(pos).is_none())
            .map(|pos| (na::Point2::new(pos.x, pos.y), 1))
            .collect()
    }
}

impl Default for Board {
    fn default() -> Self {
        let waypoints = vec![
            (na::Point2::new(5, 19), na::Point2::new(33, 19)),
            (na::Point2::new(33, 19), na::Point2::new(33, 5)),
            (na::Point2::new(33, 5), na::Point2::new(19, 5)),
            (na::Point2::new(19, 5), na::Point2::new(19, 33)),
            (na::Point2::new(19, 33), na::Point2::new(33, 33)),
        ];

        Board {
            tiles: Vec::with_capacity(BOARD_SIZE),
            mobs: Vec::with_capacity(100),
            waypoints,
        }
    }
}
