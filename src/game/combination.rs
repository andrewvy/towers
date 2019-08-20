use crate::game::unit::{Unit, UnitType};

static COMBINATIONS: &'static [(UnitType, [(UnitType, u16); 3])] = &[(
    UnitType::Knight,
    [
        (UnitType::Warrior, 1),
        (UnitType::Mage, 1),
        (UnitType::Ranger, 1),
    ],
)];

pub trait Combination {
    fn combinations(&self) -> Vec<UnitType>;
}

impl Combination for Vec<Unit> {
    fn combinations(&self) -> Vec<UnitType> {
        COMBINATIONS
            .iter()
            .filter_map(|(unit_type, requirements)| {
                if requirements
                    .iter()
                    .all(|(required_unit_type, required_rank)| {
                        self.iter().any(|unit| {
                            &unit.unit_type == required_unit_type && &unit.rank == required_rank
                        })
                    })
                {
                    Some(*unit_type)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_combinations() {
        let unit_1 = make_unit(UnitType::Warrior);
        let unit_2 = make_unit(UnitType::Warrior);
        let unit_3 = make_unit(UnitType::Warrior);
        let units = vec![unit_1, unit_2, unit_3];
        let combinations = units.combinations();

        assert!(combinations.is_empty());
    }

    #[test]
    fn can_get_combination() {
        let unit_1 = make_unit(UnitType::Warrior);
        let unit_2 = make_unit(UnitType::Mage);
        let unit_3 = make_unit(UnitType::Ranger);
        let units = vec![unit_1, unit_2, unit_3];
        let combinations = units.combinations();

        assert_eq!(vec![UnitType::Knight], combinations);
    }

    fn make_unit<'a>(unit_type: UnitType) -> Unit {
        Unit {
            unit_type: unit_type,
            ..Unit::default()
        }
    }
}
