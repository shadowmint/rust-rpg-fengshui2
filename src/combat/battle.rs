use combat::battle_character::BattleCharacter;
use characters::character::Character;
use log::game_log::GameLog;
use combat::battle_log::BattleLog;
use combat::battle_shot::BattleShot;
use combat::battle_shot::BattleShotCounter;
use combat::battle_shot::MAX_BATTLE_SHOT;

pub struct Battle<'a> {
    characters: Vec<BattleCharacter<'a>>,
    current_shot: usize,
    current_sequence: Option<usize>,
    game_log: GameLog,
}

impl<'a> Battle<'a> {
    /// Return a new empty battle
    pub fn new() -> Battle<'a> {
        return Battle {
            characters: Vec::new(),
            current_shot: MAX_BATTLE_SHOT + 1,
            current_sequence: None,
            game_log: GameLog::new(),
        };
    }

    /// Add a character to the battle
    pub fn add_character(&mut self, character: &'a Character) {
        self.game_log.log(BattleLog::AddedCharacterToBattle(character.clone()));
        let mut bc = BattleCharacter::new(character);
        match self.current_sequence {
            Some(_) => {
                bc.roll_initiative(self.current_shot, &mut self.game_log);
            }
            None => {}
        }
        self.characters.push(bc);
    }

    /// Start a new sequence, re-calculate the initiative for this shot
    pub fn start_new_sequence(&mut self) {
        let mut game_log = self.game_log.clone();
        let current_shot = self.current_shot;
        let next_sequence = match self.current_sequence {
            Some(id) => id + 1,
            None => 1
        };
        game_log.log(BattleLog::StartNewBattleSequence(next_sequence));
        self.characters.iter_mut().for_each(|c| c.roll_initiative(current_shot, &mut game_log));
        self.current_sequence = Some(next_sequence);
    }

    /// Get a summary of the current sequence
    pub fn get_shot_counter(&self) -> BattleShotCounter<'a> {
        let mut counter = BattleShotCounter::new();
        for i in (1..21).rev() {
            counter.shots.push(self.get_shot(i))
        }
        return counter;
    }

    /// Get a specific shot
    pub fn get_shot(&self, shot: usize) -> BattleShot<'a> {
        let mut rtn = BattleShot::new(shot);
        for i in self.characters.iter() {
            if i.initiative == shot {
                rtn.actors.push(i.character);
            }
        }
        return rtn;
    }

    /// Move on to the next shot, and return a list of the characters that act in it.
    /// Returns None when there are no more shots.
    pub fn next_shot(&mut self) -> Option<BattleShot<'a>> {
        let mut rtn: Option<BattleShot<'a>> = None;
        while self.current_shot > 0 && rtn.is_none() {
            let shot = self.get_shot(self.current_shot);
            if shot.actors.len() > 0 {
                rtn = Some(shot)
            }
            self.current_shot -= 1;
        }
        return rtn;
    }

    // Pick an action for a character
    //pub fn set_action(&mut self, character: &Character, action: BattleAction) {}
}

#[cfg(test)]
mod tests {
    use archetypes::thief::thief_archetype;
    use combat::battle::Battle;

    #[test]
    fn test_create_battle() {
        let fixture = thief_archetype("Mr Min");
        let mut battle = Battle::new();
        battle.add_character(&fixture);
        battle.start_new_sequence();
    }
}
