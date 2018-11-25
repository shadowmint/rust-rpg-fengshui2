use log::game_log::GameLog;
use log::game_log_entry::GameLogEntry;
use log::game_log_entry::GameLogItem;
use log::game_log_entry::GameLogItem::Message;

#[derive(Debug, Clone)]
pub struct D6 {
    /// Does a roll of 6 explode to an extra dice value?
    explodes: bool,

    /// Is this a lucky or unlucky dice?
    /// Unlucky dice result in -ve values
    lucky: bool,

    /// Did this dice explode when you rolled it?
    /// The count of the number of times it exploded.
    exploded: usize,

    /// What was the output of this roll?
    value: Option<usize>,
}

impl D6 {
    /// Return a single unmodified, non-exploding dice value
    pub fn single() -> D6 {
        return D6 {
            explodes: false,
            lucky: true,
            exploded: 0,
            value: None,
        };
    }

    /// Resolve this dice roll
    /// For complex use cases (eg. check for rail cars) examine the object afterwards
    pub fn resolve(&mut self, game: &mut GameLog) -> usize {
        let mut value: usize = 0;
        while value % 6 == 0 {
            let roll = rand::random::<u8>() as usize % 6 + 1;
            value += roll;
            if roll == 6 {
                if self.explodes {
                    self.exploded += 1;
                } else {
                    break;
                }
            }
        }
        self.value = Some(value);
        game.log(self.clone());
        return value;
    }
}

impl GameLogEntry for D6 {
    fn message(&self) -> GameLogItem {
        if self.exploded > 0 {
            return Message(format!("Rolled D6 -> {:?} (EXPLODED {:?} times)", self.value.unwrap_or(0), self.exploded));
        } else {
            return Message(format!("Rolled D6 -> {:?}", self.value.unwrap_or(0)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::D6;
    use log::game_log::GameLog;

    #[test]
    fn test_roll_simple_d6() {
        let mut game = GameLog::new();
        for _ in 0..10 {
            let output = D6::single().resolve(&mut game);
            assert!(output >= 1 && output < 7);
        }
    }
}
