#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut n_red = 0;
        let mut n_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => n_red += 1,
                ShirtColor::Blue => n_blue += 1,
            }
        }

        if n_red > n_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn closure_type_inference() {
    let closure_1 = |x| x;
    println!("{}", closure_1(String::from("a")));
    // The following is not possible. The types inferred by the compiler
    // from the first call to the closure get locked-in.
    // println!("{}", closure_2(29));
}

fn immutable_borrow_closure() {
    println!("Immutable borrow example");
    let list = vec![1, 2, 3];
    println!("Before defining the closure. List: {list:?}");

    let borrow_immutably = || println!("Inside the closure. List: {list:?}");

    println!("Before calling the closure. List: {list:?}");
    borrow_immutably();
    println!("After calling the closure. List: {list:?}");
}

fn mutable_borrow_closure() {
    println!("Mutable borrow example");
    let mut list1 = vec![1, 2, 3];
    println!("Before defining the closure. List: {list1:?}");

    let mut borrow_mutably = || list1.push(5);

    // The following statement causes compiler error if uncommented.
    // println!("Before calling the closure. List: {list1:?}");
    borrow_mutably();
    println!("After calling the closure. List: {list1:?}");
}

fn other_examples() {
    closure_type_inference();
    immutable_borrow_closure();
    mutable_borrow_closure();
}

fn main() {
    let shirts = vec![
        ShirtColor::Red,
        ShirtColor::Red,
        ShirtColor::Blue,
        ShirtColor::Blue,
        ShirtColor::Blue,
    ];
    let store = Inventory { shirts };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    other_examples();
}
