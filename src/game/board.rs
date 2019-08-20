use crate::game::unit::Unit;
use ggez::nalgebra as na;

pub struct Board {
    pub tiles: Vec<Unit>,
}

const BOARD_HEIGHT: usize = 15;
const BOARD_WIDTH: usize = 15;
const BOARD_SIZE: usize = BOARD_HEIGHT * BOARD_WIDTH;

impl Board {
    pub fn at_position<'a>(&'a self, coordinates: na::Point2<usize>) -> &'a Unit {
        &self.tiles[coordinates.x * coordinates.y]
    }

    pub fn with_positions<'a>(&'a self) -> Vec<(na::Point2<u32>, &'a Unit)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(index, unit)| {
                (
                    na::Point2::new((index % BOARD_HEIGHT) as u32, (index / BOARD_WIDTH) as u32),
                    unit,
                )
            })
            .collect()
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            tiles: Vec::with_capacity(BOARD_SIZE),
        }
    }
}
