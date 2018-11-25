pub struct Character {
    pub stats: Stats
}

pub struct Stats {
    pub martial_arts: usize,
    pub backup_attack: BackupAttack,
    pub defence: usize,
    pub toughness: usize,
    pub fortune: usize,
    pub speed: usize,
}

pub enum BackupAttack {
    Guns(usize)
}