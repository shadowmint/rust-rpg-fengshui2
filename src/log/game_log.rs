use base_logging::Level;
use log::game_logger::GameLogger;
use log::game_log_entry::GameLogEntry;
use log::game_logger::GameLogFormatter;
use base_logging::LoggerRef;
use log::game_log_entry::GameLogItem::Message;
use log::game_log_entry::GameLogItem::Messages;

#[derive(Clone)]
pub struct GameLog {
    logger: LoggerRef
}

impl GameLog {
    pub fn new() -> GameLog {
        return GameLog {
            logger: LoggerRef::new()
                .with_level(Level::Info)
                .with_format(GameLogFormatter {})
                .with(GameLogger {})
        };
    }

    pub fn log(&mut self, event: impl GameLogEntry) {
        match event.message() {
            Message(m) => self.logger.log(Level::Info, m),
            Messages(m) => {
                for msg in m.iter() {
                    self.logger.log(Level::Info, msg.to_string())
                }
            }
        }
    }
}