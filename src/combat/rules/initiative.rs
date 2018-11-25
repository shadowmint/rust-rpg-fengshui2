use rolls::d6::D6;
use characters::character::Character;
use log::game_log::GameLog;
use combat::rules::rule_log::RuleLog;

// ref p. 100
pub fn rule_derive_initiative(character: &Character, max: usize, game: &mut GameLog) -> usize {
    let rolled = D6::single().resolve(game);
    let speed = character.stats.speed;
    let mut result = speed + rolled;
    game.log(RuleLog::RolledInitiative(character, rolled, speed, result));

    // If we are on say, shot 7 when a character joins, they can't magically get an init of 20
    if result > max {
        game.log(RuleLog::CappedInitiative(result, max));
        result = max;
    }

    return result;
}