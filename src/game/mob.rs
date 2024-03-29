use std::time::Instant;

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

#[derive(Debug, PartialEq, Clone)]
pub enum MobEntityStatus {
    Walking,
    FinishedPath,
}

#[derive(Debug, Clone)]
pub struct MobEntity {
    pub position: na::Point2<f32>,
    pub last_position: na::Point2<f32>,
    pub destination: na::Point2<f32>,
    pub path_index: u32,
    pub movement_speed: f32,
    pub max_health: i32,
    pub current_health: i32,
    pub physical_defense: i32,
    pub magical_defense: i32,
    pub invisible: bool,
    pub status: MobEntityStatus,
    pub last_damaged_at: Instant,
}

impl MobEntity {
    pub fn update(&mut self, dt: f32) {
        if self.status != MobEntityStatus::FinishedPath {
            // @TODO(vy): This detects whether the mob is at the designated tile. These magic
            // numbers are hardcoded, which represents that a tile is 16px wide & high.
            let world_destination =
                na::Point2::<f32>::new(self.destination.x * 16.0, self.destination.y * 16.0);
            let distance = na::distance(&self.position, &world_destination);
            if distance < 1.0 {
                self.status = MobEntityStatus::FinishedPath;
            } else {
                let diff: na::Vector2<f32> = world_destination - self.position;
                let new_position = self.position + diff.normalize() * self.movement_speed * dt;

                self.last_position = self.position;
                self.position = new_position;
            }
        }
    }

    pub fn damage(&mut self, damage: u32) {
        self.current_health -= damage as i32;
        self.last_damaged_at = Instant::now();
    }

    pub fn show_health_bar(&self) -> bool {
        self.last_damaged_at.elapsed().as_secs() < 5
    }

    pub fn is_alive(&self) -> bool {
        self.current_health > 0
    }
}

impl From<&MobDefinition> for MobEntity {
    fn from(definition: &MobDefinition) -> Self {
        MobEntity {
            position: na::Point2::new(80.0, 80.0),
            last_position: na::Point2::new(5.0, 19.0),
            destination: na::Point2::new(5.0, 19.0),
            path_index: 0,
            status: MobEntityStatus::Walking,
            max_health: definition.health,
            current_health: definition.health,
            physical_defense: definition.physical_defense,
            magical_defense: definition.magical_defense,
            invisible: definition.invisible,
            movement_speed: 50.0,
            last_damaged_at: Instant::now(),
        }
    }
}
