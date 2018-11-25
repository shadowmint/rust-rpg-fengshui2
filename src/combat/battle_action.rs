use characters::character::Character;

pub enum BattleAction<'a> {
    Attack(&'a Character)
}