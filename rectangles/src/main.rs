#[derive(Debug)]    // derived "Debug" trait. Helps us print ({val:?}) or pretty print ({val:#?}) to stdout or
                    // stderr with println!(&val) or dbg!(&val)
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // A method (a kind of associated function) that returns the area of
    // this rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // A method (a kind of associated function) that takes in &self as the
    // first argument and returns a boolean based on if the argument rect
    // can fit inside this rectangle or not.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // An associated function that doesn't take self as the first argument.
    fn as_square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let multiplier = 5;
    let rect1 = Rectangle {
        width: dbg!(multiplier * 20),   // Possible since dbg macro returns the ownership back to
                                        // the caller.
        height: dbg!(multiplier * 30),
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30
    };

    let rect3 = Rectangle {
        width: 18,
        height: 25,
    };

    dbg!(&rect1);   // Outputs to stderr. prints file, line#, param name and value. Takes ownership
                    // and gives it back too.

    dbg!(&rect2);
    dbg!(&rect3);
    // println macro outputs to stdout. Does not take ownership of the data, only reference.
    println!("Area of rect1 with w={} and h={} = {} sq. units.", rect1.width, rect1.height, rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect3: {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect1: {}", rect3.can_hold(&rect1));

    let side = 5;
    let sq = Rectangle::as_square(side);

    println!("Created a square with side: {side}");
    dbg!(&sq);
}

