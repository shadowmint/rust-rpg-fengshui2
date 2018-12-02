extern crate rust_rpg_fengshui2;

use rust_rpg_fengshui2::archetypes::thief::thief_archetype;
use rust_rpg_fengshui2::combat::battle::Battle;
use rust_rpg_fengshui2::characters::character::AttackType;
use rust_rpg_fengshui2::enemies::mook::enemy_mook;
use rust_rpg_fengshui2::log::game_log::GameLog;

pub fn main() {
    let mut game = GameLog::new();

    let mr1 = thief_archetype("Mr 1");

    let mook1 = enemy_mook("Imperial solider", AttackType::MartialArts);
    let mook2 = enemy_mook("Imperial solider", AttackType::MartialArts);
    let mook3 = enemy_mook("Imperial solider", AttackType::MartialArts);

    let mut battle = Battle::new();

    battle.add_character(&mr1);

    battle.add_character(&mook1);
    battle.add_character(&mook2);
    battle.add_character(&mook3);

    battle.start_new_sequence();

    println!("");
    game.log(battle.get_shot_counter());

    println!("");
    loop {
        match battle.next_shot() {
            Some(s) => {
                game.log(s);
                match s.actors.iter().find(|i| i == mr1) {
                    Some(actor) => {
                        battle.perform_action(actor, )
                    }
                    None => {}
                }
            }
            None => break
        }
    }
}