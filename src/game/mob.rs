use ggez::nalgebra as na;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MobDefinition {
    pub health: i32,
    pub physical_defense: i32,
    pub magical_defense: i32,
    pub invisible: bool,
    pub spritesheet_id: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MobEntity<'a> {
    pub position: na::Point2<f64>,
    pub current_health: i32,
    pub physical_defense: i32,
    pub magical_defense: i32,
    pub invisible: bool,
    pub definition: &'a MobDefinition,
}

impl<'a> MobEntity<'a> {
    #[allow(dead_code)]
    pub fn new(definition: &'a MobDefinition) -> Self {
        Self {
            position: na::Point2::new(0.0, 0.0),
            current_health: definition.health,
            physical_defense: definition.physical_defense,
            magical_defense: definition.magical_defense,
            invisible: definition.invisible,
            definition,
        }
    }
}
