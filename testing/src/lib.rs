#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(i: u64) -> u64 {
    i + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // arrange
        let larger = Rectangle {
            width: 10,
            height: 8,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        // act
        let result = larger.can_hold(&smaller);

        // assert
        assert!(result)
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        // arrange
        let larger = Rectangle {
            width: 10,
            height: 8,
        };

        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        // act
        let result = !smaller.can_hold(&larger);

        // assert
        assert!(result)
    }

    #[test]
    fn adds_two() -> Result<(), String> {
        let result = add_two(5);

        if result == 7 {
            Ok(())
        } else {
            Err(String::from("5 + 2 does not equal 7"))
        }
    }
}
