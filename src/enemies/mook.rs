use characters::character::Character;
use characters::character::Stats;
use archetypes::archetype::Archetype;
use characters::character::CharacterType;
use characters::character::Attack::*;
use characters::character::Fortune::*;
use characters::character::AttackType;

pub fn enemy_mook(name: &str, attack_type: AttackType) -> Character {
    return Character {
        character_type: CharacterType::Mook,
        name: name.to_string(),
        archetype: Archetype::None,
        stats: Stats {
            attack: AttackSome(attack_type, 13),
            backup_attack: AttackNone,
            defence: 13,
            toughness: 0,
            fortune: FortuneNone,
            speed: 5,
        },
    };
}

#[cfg(test)]
mod tests {
    use super::enemy_mook;
    use characters::character::AttackType;

    #[test]
    fn test_enemy_mook() {
        let _ = enemy_mook("Imperial guard", AttackType::MartialArts);
    }
}
