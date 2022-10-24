extern crate core;

use std::io;

const SCALE_CELSIUS: u8 = 1;
const SCALE_FAHRENHEIT: u8 = 2;

fn main() {
    let mut empty_string = "".to_string();

    println!("This is °C <-> °F converter!");
    println!("What scale would you to convert?");

    loop {
        println!("Set 1 for °C and 2 for °F:");

        let mut scale = String::new();
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");
        let scale: u8 = scale.trim().parse().expect("Please type a number!");

        println!("Set temperature:");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line!");
        let temperature: f32 = temperature.trim().parse().expect("Please type a number!");

        let scale_from: char;
        let scale_to: char;
        let converted_temp: f32;

        match scale {
            SCALE_CELSIUS => {
                scale_from = 'C';
                scale_to = 'F';
                converted_temp = convert_celsius_fahrenheit(temperature);
            }
            SCALE_FAHRENHEIT => {
                scale_from = 'F';
                scale_to = 'C';
                converted_temp = convert_fahrenheit_celsius(temperature);
            }
            _ => {
                println!("Invalid scale!");
                continue;
            }
        }

        println!("{}°{} -> {:.2}°{}", temperature, scale_from, converted_temp, scale_to);

        println!("Continue? [Y/n]");
        let mut cmd = String::new();
        let res = io::stdin()
            .read_line(&mut cmd);

        match res {
            Ok(_) => {
                if cmd.trim().eq_ignore_ascii_case("y") || cmd.trim().eq(&mut empty_string) {
                    continue;
                }
                if cmd.trim().eq_ignore_ascii_case("n") {
                    println!("Done, exiting.");
                    break;
                } else {
                    println!("Invalid input, exiting {}.", cmd);
                    break;
                }
            },
            Err(_) => continue,
        }
    };
}

fn convert_celsius_fahrenheit(temperature: f32) -> f32 {
    temperature * (9.0 / 5.0) + 32.0
}

fn convert_fahrenheit_celsius(temperature: f32) -> f32 {
    (temperature - 32.0) * (5.0 / 9.0)
}
