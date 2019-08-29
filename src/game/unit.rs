use std::time::Instant;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum UnitType {
    Warrior,
    Mage,
    Cleric,
    Ranger,
    Scout,
    Knight,
    DarkKnight,
    HeavyKnight,
    Android,
    Sorcerer,
    Geomancer,
    MagicKnight,
    Clergy,
    Sage,
    Celestial,
    Archer,
    Gunner,
    Gunslinger,
    Valkyrie,
    Wall,
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub range: f32,
    pub damage: u32,
    pub attack_speed: f32,
    pub unit_type: UnitType,
    pub rank: u16,
    pub attacks: bool,

    #[serde(skip, default = "Instant::now")]
    pub last_attacked: Instant,
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            range: 0.0,
            damage: 10,
            attack_speed: 2.0,
            unit_type: UnitType::Wall,
            rank: 1,
            attacks: false,
            last_attacked: Instant::now(),
        }
    }
}

impl Unit {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Unit::default()
    }

    pub fn attack_speed(&self) -> u128 {
        (((1.0 / self.attack_speed) * 1000.0) as u32).into()
    }

    pub fn check_attack(&self) -> Option<u32> {
        let attack_speed = self.attack_speed();

        if self.last_attacked.elapsed().as_millis() >= attack_speed {
            return Some(self.damage);
        } else {
            return None;
        }
    }

    pub fn perform_attack(&mut self) {
        self.last_attacked = Instant::now();
    }
}
