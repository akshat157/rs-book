use std::{
    fs,
    fs::File,
    io::{self, ErrorKind, Write},
};

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_text_from_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeting_file_path = "hello.txt";
    let greeting_file_result = File::open(greeting_file_path);

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(greeting_file_path) {
                Ok(mut fc) => {
                    let text = "Hello from Rust!";
                    println!("Writing text to file: \"{text}\"");
                    fc.write_all(text.as_bytes())
                        .expect("Failed to write to file");
                    fc
                }
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    let file_text = match read_text_from_file(greeting_file_path) {
        Ok(text) => text,
        Err(e) => panic!("Error reading file text: {e:?}"),
    };

    let last_char_of_first_line = last_char_of_first_line(&file_text)
        .expect("Couldn't get the last character of the first line.");

    println!("text read from file: \"{file_text}\"");
    println!("last char of the first line in the text is: \'{last_char_of_first_line}\'");

    Ok(())
}
