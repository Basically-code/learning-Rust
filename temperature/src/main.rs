/* 
    - The program should ask a user for the temperature 
    - Ask which units the temperature is in
    - Ask which temperature to convert to (Celsius, Fahrenheit, kelvin)
    - Output a string with the respective 
*/

use std::io;

fn main() {
    println!("Welcome to Mr Themometer enter the temperature and Mr T will get to action");

    let temperature = input("Enter the temperature (Without the symbol) and (as a float: 00.0)", "Failed to read line", true);


    let degree = loop {
        match input("Enter the degree of that temperature (C,F,K)", "Failed to read line", false).trim() {
            "C" => break "C",
            "F" => break "F",
            "K" => break "K",
            _   => {
                continue;
            }
        };
    };

    let mut c  = 0.0;
    let mut f  = 0.0;
    let mut k  = 0.0;

    let temperature: f32 = temperature.trim().parse().expect("Input was not a number");

    match degree.trim() {
        "C" => {
            c  = temperature;
            f  = fahrenheit(temperature);
            k  = kelvin(temperature);
        },
        "F" => {
            c = celsius_f(temperature);
            f = temperature;
            k = kelvin(celsius_f(temperature));
        },
        "K" => {
            c = celsius_k(temperature);
            f = fahrenheit(celsius_k(temperature));
            k = temperature
        }
        _ => {
            println!("Please enter either (C, F, K)")
        }
    }

    println!("The temperature in Celsius: {c}°C");
    println!("The temperature in Fahrenheit: {f}°F");
    println!("The temperature in Kelvin: {k}°K");
}

fn fahrenheit(c: f32) -> f32 {
    c * (9.0 / 5.0) + 32.0
}

fn kelvin(c: f32) -> f32 {
    c + 273.15
}

fn celsius_f(f: f32) -> f32 {
    (f - 32.0) * (5.0 / 9.0)
}

fn celsius_k(k: f32) -> f32 {
    k - 273.15
}

fn input(message: &str, err: &str, num: bool) -> String {
    let mut temp = String::new();
    loop {
        loop {
            println!("{message}");
            match io::stdin().read_line(&mut temp) {
                Ok(_) => {
                    break;
                },
                Err(_) => {println!("{err}"); continue; },
            };
        }

        if num {
            match temp.trim().parse::<f32>(){
                Ok(_)  => return temp,
                Err(_) => continue,
            }
        }else {
            return temp
        }
    }
    
    
}
