extern crate rand;
extern crate time;
extern crate base_logging;

pub mod archetypes;
pub mod characters;
pub mod combat;
pub mod rolls;
pub mod log;
pub mod enemies;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
