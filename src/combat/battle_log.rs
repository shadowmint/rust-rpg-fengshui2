use characters::character::Character;
use log::game_log_entry::GameLogEntry;
use log::game_log_entry::GameLogItem;
use log::game_log_entry::GameLogItem::Message;

pub enum BattleLog {
    AddedCharacterToBattle(Character),
    StartNewBattleSequence(usize),
}

impl GameLogEntry for BattleLog {
    fn message(&self) -> GameLogItem {
        match self {
            BattleLog::AddedCharacterToBattle(c) => {
                Message(format!("Added {} ({:?}) to battle", c.name, c.character_type))
            }
            BattleLog::StartNewBattleSequence(v) => {
                Message(format!("Starting battle sequence {}", v))
            }
        }
    }
}