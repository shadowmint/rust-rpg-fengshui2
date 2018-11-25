use characters::character::Character;
use log::game_log_entry::GameLogEntry;
use log::game_log_entry::GameLogItem::Message;
use log::game_log_entry::GameLogItem;

pub enum RuleLog<'a> {
    RolledInitiative(&'a Character, usize, usize, usize),
    CappedInitiative(usize, usize),
}

impl<'a> GameLogEntry for RuleLog<'a> {
    fn message(&self) -> GameLogItem {
        match self {
            RuleLog::RolledInitiative(c, raw, speed, value) => {
                Message(format!("{} rolled {} + speed {} for initiative -> {}", c.name, raw, speed, value))
            }
            RuleLog::CappedInitiative(rolled, max) => {
                Message(format!("Initiative of {} was capped to the max of {}", rolled, max))
            }
        }
    }
}