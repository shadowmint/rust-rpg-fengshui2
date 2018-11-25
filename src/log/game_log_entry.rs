pub enum GameLogItem {
    Message(String),
    Messages(Vec<String>),
}

pub trait GameLogEntry {
    fn message(&self) -> GameLogItem;
}