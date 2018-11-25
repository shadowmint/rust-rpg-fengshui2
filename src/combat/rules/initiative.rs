use rolls::d6::D6;
use characters::character::Character;

// ref p. 100
pub fn rule_derive_initiative(character: &Character) -> usize {
    return character.stats.speed + D6::single().resolve();
}