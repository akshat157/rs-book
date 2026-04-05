use std::{
    fs::File,
    io::{self, ErrorKind, Read, Write},
};

fn read_text_from_file(filename: &str) -> Result<String, io::Error> {
    let text_file_result = File::open(filename);

    let mut text_file = match text_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut text = String::new();

    match text_file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(mut fc) => {
                    println!("Writing text to file: \"Rust wrote this text!\n\"");
                    fc.write_all("Rust wrote this text!\n".as_bytes())
                        .expect("Failed to write to file");
                    fc
                }
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    let file_text = match read_text_from_file("hello.txt") {
        Ok(text) => text,
        Err(e) => panic!("Error reading file text: {e:?}"),
    };

    println!("text read from file:");
    print!("{file_text}");
}
