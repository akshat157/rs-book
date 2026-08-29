#[cfg(test)]
use crate::guess::Guess;

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(101);
}

#[test]
#[should_panic(expected = "greater than or equal to 1")]
fn less_than_1() {
    Guess::new(0);
}
