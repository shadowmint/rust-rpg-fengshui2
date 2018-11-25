extern crate rand;

pub mod archetypes;
pub mod characters;
pub mod combat;
pub mod rolls;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
