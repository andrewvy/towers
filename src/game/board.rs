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
        let results = self.waypoints.iter().map(|(start, end)| {
            self.calculate_path(start, end)
        });

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
