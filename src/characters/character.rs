use archetypes::archetype::Archetype;

#[derive(Debug, Clone)]
pub struct Character {
    pub character_type: CharacterType,
    pub name: String,
    pub archetype: Archetype,
    pub stats: Stats,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub attack: Attack,
    pub backup_attack: Attack,
    pub defence: usize,
    pub toughness: usize,
    pub fortune: Fortune,
    pub speed: usize,
}

#[derive(Debug, Clone)]
pub enum Attack {
    AttackNone,
    AttackSome(AttackType, usize),
}

#[derive(Debug, Clone)]
pub enum AttackType {
    MartialArts,
    Guns,
    Sorcery,
    ScroungeTech,
    CreaturePowers,
    Mutant,
}

#[derive(Debug, Clone)]
pub enum Fortune {
    FortuneNone,
    FortuneSome(FortuneType, usize)
}

#[derive(Debug, Clone)]
pub enum FortuneType {
    Fortune,
    Chi,
    Genome,
    Magic,
}

#[derive(Debug, Clone)]
pub enum CharacterType {
    PlayerCharacter,
    Mook,
}