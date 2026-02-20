use std::io;

fn main() {
    print_intro();
    loop {
        print_menu();

        let mut opt = String::new();

        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");

        let opt: u8 = opt
            .trim()
            .parse()
            .expect("Option entered is invalid. Expected a number among 0, 1 and 2.");

        if opt > 2 {
            println!("Invalid option. Expected 0, 1 or 2.");
            continue;
        }

        if opt == 0 {
            println!("Exiting.");
            break;
        }

        println!("Enter the temperature value for conversion:");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = temperature
            .trim()
            .parse()
            .expect("Invalid input. Expected a number!");

        let converted: f32;
        if opt == 1 {
            println!("\nConverting Celsius to Fahrenheit...");
            // Celsius to Farenheit
            converted = celsius_to_farenheit(temperature);
        } else {
            // Farenheit to Celsius
            println!("\nConverting Fahrenheit to Celsius...");
            converted = farenheit_to_celsius(temperature);   
        }

        let (input_unit, res_unit): (char, char) = if opt == 1 { ('C', 'F') } else { ('F', 'C') };

        println!("{temperature} deg {input_unit} = {converted} deg {res_unit}");
    }
}

fn celsius_to_farenheit(c: f32) -> f32 {
    (9.0 * c) / 5.0 + 32.0
}

fn farenheit_to_celsius(f: f32) -> f32 {
    5.0 * (f - 32.0) / 9.0
}

fn print_intro() {
        println!();
        println!("----------------------------------------");
        println!("| Akshat's Farenheit-Celsius Converter |");
        println!("----------------------------------------");
}

fn print_menu() {
        println!("Choose the operation you'd like to perform");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!();
        println!("0: Quit the program");
        println!();
} 
