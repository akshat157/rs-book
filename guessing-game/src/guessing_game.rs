pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be in the range 1 to 100, both inclusive.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
