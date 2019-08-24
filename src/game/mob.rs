use ggez::nalgebra as na;
use serde::Deserialize;

// @TODO(vy): remove copy
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct MobDefinition {
    pub health: i32,
    pub physical_defense: i32,
    pub magical_defense: i32,
    pub invisible: bool,
    pub spritesheet_id: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MobEntity {
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    pub current_health: i32,
    pub physical_defense: i32,
    pub magical_defense: i32,
    pub invisible: bool,
}

impl MobEntity {
    #[allow(dead_code)]
    pub fn new(definition: &MobDefinition) -> Self {
        Self {
            position: na::Point2::new(0.0, 0.0),
            velocity: na::Vector2::new(0.1, 0.1),
            current_health: definition.health,
            physical_defense: definition.physical_defense,
            magical_defense: definition.magical_defense,
            invisible: definition.invisible,
        }
    }
}

impl From<MobDefinition> for MobEntity {
    fn from(definition: MobDefinition) -> Self {
        MobEntity {
            position: na::Point2::new(0.0, 0.0),
            velocity: na::Vector2::new(0.05, 0.05),
            current_health: definition.health,
            physical_defense: definition.physical_defense,
            magical_defense: definition.magical_defense,
            invisible: definition.invisible,
        }
    }
}
