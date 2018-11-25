use characters::character::Character;
use characters::character::Stats;
use characters::character::BackupAttack;

pub fn thief_archetype() -> Character {
    return Character {
        stats: Stats {
            martial_arts: 13,
            backup_attack: BackupAttack::Guns(12),
            defence: 16,
            toughness: 6,
            fortune: 6,
            speed: 9,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::thief_archetype;

    #[test]
    fn test_thief_archetype() {
        let _ = thief_archetype();
    }
}
