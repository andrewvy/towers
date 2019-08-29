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
            damage: 0,
            attack_speed: 1.0,
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

    pub fn perform_attack(&mut self) -> bool {
        let attack_per_millis = ((1.0 / self.attack_speed) as u32 * 1000).into();

        if self.last_attacked.elapsed().as_millis() >= attack_per_millis {
            self.last_attacked = Instant::now();
            return true;
        } else {
            return false;
        }
    }
}
