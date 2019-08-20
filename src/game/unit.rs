#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Unit {
    pub range: f32,
    pub damage: u32,
    pub attack_speed: f32,
    pub unit_type: UnitType,
    pub rank: u16,
    pub attacks: bool,
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            range: 0.0,
            damage: 0,
            attack_speed: 0.0,
            unit_type: UnitType::Wall,
            rank: 1,
            attacks: false,
        }
    }
}

impl Unit {
    pub fn new() -> Self {
        Unit::default()
    }
}
