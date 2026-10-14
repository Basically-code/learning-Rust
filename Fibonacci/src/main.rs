/* 
    It should take in a positive number finding the nth fibonacci number
    Output a string showing the nth number
*/
use std::io;

fn main() {
    
    println!("Welcome to Leonardo Bonacci's fibonacci sequence");

    println!("Which fibonacci number do you want");

    let n = loop {
        let mut text = String::new();
        if io::stdin().read_line(&mut text).is_err() {
            println!("Please try again, There was an invalid input");
            continue;
        } 

        match text.trim().parse::<u64>() {
            Ok(num)  => break num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
    };

    let nth = fibonacci(n);
    println!("The {n}th fibonacci number is: {nth}");
}


fn fibonacci(n: u64) -> u64 {
    if n == 0 {return 0;}
    if n == 1 {return 1;}

    let mut n1 = 0;
    let mut n2 = 1;
    let mut cnt = 1;

    loop {
        if cnt == n {return n2;}
        cnt += 1;

        let temp = n1;
        n1 = n2;
        n2 += temp;
    }
}
