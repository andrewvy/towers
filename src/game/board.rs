use crate::game::unit::Unit;

pub struct Board {
    pub tiles: Vec<Unit>,
}

const BOARD_SIZE: usize = 15 * 15;
impl Default for Board {
    fn default() -> Self {
        Board {
            tiles: Vec::with_capacity(BOARD_SIZE)
        }
    }
}