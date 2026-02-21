fn main() {
    let s = String::from("hello");
    let i = 0; // Can set to any arbitrary value.

    if i % 2 == 0 {
        take_ownership(s);
    } else {
        println!("{i}");
    }

    // If uncommented, this means that we try to use `s` after its ownership has
    // been transfered to `take_ownership`. Though, if left commented, the compiler does not have a
    // problem because there's no potential `Error: use of moved value` error.
    // println!("{s}");  
}

fn take_ownership(str: String) {
    println!("{str}");
}

