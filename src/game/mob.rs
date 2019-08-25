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
    pub destination: na::Point2<f32>,
    pub path_index: u32,
    pub movement_speed: f32,
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
            destination: na::Point2::new(1.0, 1.0),
            path_index: 0,
            current_health: definition.health,
            physical_defense: definition.physical_defense,
            magical_defense: definition.magical_defense,
            invisible: definition.invisible,
            movement_speed: 1.0,
        }
    }

    pub fn update(&mut self) {
        let diff: na::Vector2<f32> = self.destination - self.position;
        let new_position = self.position + diff.normalize() * self.movement_speed;
        self.position = new_position;
    }
}

impl From<MobDefinition> for MobEntity {
    fn from(definition: MobDefinition) -> Self {
        MobEntity {
            position: na::Point2::new(0.0, 0.0),
            destination: na::Point2::new(200.0, 200.0),
            path_index: 0,
            current_health: definition.health,
            physical_defense: definition.physical_defense,
            magical_defense: definition.magical_defense,
            invisible: definition.invisible,
            movement_speed: 3.0,
        }
    }
}
