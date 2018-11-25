use base_logging::Log;
use base_logging::LogFormatter;
use std::collections::HashMap;
use base_logging::Level;
use time::Tm;

pub struct GameLogger {}

impl Log for GameLogger {
    fn log(&mut self, message: &str) {
        println!("{}", message);
    }
}

pub struct GameLogFormatter {}

impl LogFormatter for GameLogFormatter {
    fn log_format(&self, level: Level, timestamp: Tm, message: Option<&str>, properties: Option<HashMap<&str, &str>>) -> String {
        let time = match time::strftime("%b %d: %H:%M:%S:", &timestamp) {
            Ok(i) => i,
            Err(_) => String::from("")
        };
        format!("{} {:?}: FengShui: {}", time, level, self.combine(message, properties))
    }
}

impl GameLogFormatter {
    fn combine(&self, message: Option<&str>, properties: Option<HashMap<&str, &str>>) -> String {
        let mut rtn = String::new();
        match message {
            Some(msg) => {
                rtn.push_str(msg);
            }
            None => {}
        };
        match properties {
            Some(props) => {
                for (key, value) in props.iter() {
                    rtn.push_str(&format!(" {}:{}", key, value));
                }
            }
            None => {}
        }
        return rtn;
    }
}