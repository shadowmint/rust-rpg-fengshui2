use characters::character::Character;
use log::game_log_entry::GameLogEntry;
use log::game_log_entry::GameLogItem;
use log::game_log_entry::GameLogItem::Messages;
use std::fmt;

/// This is the flat out maximum value any initiative value can have.
pub const MAX_BATTLE_SHOT: usize = 20;

/// A single shot in the current battle
pub struct BattleShot<'a> {
    pub shot_number: usize,
    pub actors: Vec<&'a Character>,
}

/// The shot counter is a snapshot of the current state of the sequence
pub struct BattleShotCounter<'a> {
    pub shots: Vec<BattleShot<'a>>
}

impl<'a> BattleShot<'a> {
    pub fn new(shot: usize) -> BattleShot<'a> {
        return BattleShot {
            shot_number: shot,
            actors: Vec::new(),
        };
    }
}

impl<'a> BattleShotCounter<'a> {
    pub fn new() -> BattleShotCounter<'a> {
        return BattleShotCounter {
            shots: Vec::new(),
        };
    }
}

impl<'a> fmt::Display for BattleShot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names = self.actors.iter().map(|c| c.name.clone()).collect::<Vec<String>>().join(", ");
        write!(f, "Shot {: >2}: {}", self.shot_number, names)
    }
}

impl<'a> GameLogEntry for BattleShotCounter<'a> {
    fn message(&self) -> GameLogItem {
        let mut output = Vec::new();
        output.push(format!("Battle Shot Counter!"));
        for shot in self.shots.iter() {
            output.push(format!("! {}", shot));
        }
        return Messages(output);
    }
}