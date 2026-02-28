fn main() {
    use std::collections::HashMap;

    let text = "hello hello world wonderful amazing world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        println!("found {word}");   // word is still available to be used in this scope.
        *count += 1;
    }

    println!("string: \"{text}\" has the following frequency:\n{map:?}");
}
