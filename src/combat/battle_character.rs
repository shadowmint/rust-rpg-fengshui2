use characters::character::Character;
use combat::rules::initiative::rule_derive_initiative;

pub struct BattleCharacter<'a> {
    pub character: &'a Character,
    pub initiative: usize,
}

impl<'a> BattleCharacter<'a> {
    pub fn new(character: &'a Character) -> BattleCharacter<'a> {
        return BattleCharacter {
            character,
            initiative: 1,
        };
    }

    /// Update the init value for this character
    pub fn roll_initiative(&mut self) {
        self.initiative = rule_derive_initiative(self.character);
    }
}

#[cfg(test)]
mod tests {
    use super::BattleCharacter;
    use archetypes::thief::thief_archetype;

    #[test]
    fn test_create_battle_character() {
        let fixture = thief_archetype();
        let _ = BattleCharacter::new(&fixture);
    }
}
