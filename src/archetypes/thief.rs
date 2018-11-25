use characters::character::Character;
use characters::character::Stats;
use archetypes::archetype::Archetype;
use characters::character::CharacterType;
use characters::character::Attack::AttackSome;
use characters::character::AttackType::*;
use characters::character::FortuneType::Fortune;
use characters::character::Fortune::FortuneSome;

pub fn thief_archetype(name: &str) -> Character {
    return Character {
        character_type: CharacterType::PlayerCharacter,
        name: name.to_string(),
        archetype: Archetype::Thief,
        stats: Stats {
            attack: AttackSome(MartialArts, 13),
            backup_attack: AttackSome(Guns, 12),
            defence: 16,
            toughness: 6,
            fortune: FortuneSome(Fortune, 6),
            speed: 9,
        },
    };
}

#[cfg(test)]
mod tests {
    use super::thief_archetype;

    #[test]
    fn test_thief_archetype() {
        let _ = thief_archetype("Sammy Sam");
    }
}
