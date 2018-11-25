use combat::battle_character::BattleCharacter;
use characters::character::Character;

pub struct Battle<'a> {
    characters: Vec<BattleCharacter<'a>>
}

impl<'a> Battle<'a> {
    /// Return a new empty battle
    pub fn new() -> Battle<'a> {
        return Battle {
            characters: Vec::new()
        };
    }

    /// Add a character to the battle
    pub fn add_character(&mut self, character: &'a Character) {
        self.characters.push(BattleCharacter::new(character));
    }

    /// Start a new sequence, re-calculate the initiative for this shot
    pub fn start_sequence(&mut self) {
        self.characters.iter_mut().for_each(|c| c.roll_initiative())
    }
}

#[cfg(test)]
mod tests {
    use archetypes::thief::thief_archetype;
    use combat::battle::Battle;

    #[test]
    fn test_create_battle() {
        let fixture = thief_archetype();
        let mut battle = Battle::new();
        battle.add_character(&fixture);
        battle.start_sequence();
    }
}
