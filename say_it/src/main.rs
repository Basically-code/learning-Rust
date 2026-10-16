/*
    Input a valid number from -1_000_000_000 to 1_000_000_000 (i32)
    Output a string with the corresponding word value of the input number 
    As easy as say it
*/

use std::io;

pub const ONES: [&str; 10]  = ["zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
pub const TEENS:[&str; 10]  = ["ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","nineteen"];
pub const TENS: [&str; 10]  = ["_","_","twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

fn main() {
    let number: i32 = input("Please input the number to be translated(-1,000,000,000 to 1,000,000,000)");

    println!("{}",interpreter(number));
}

fn input(message: &str) -> i32 {
    println!("{message}");
    loop {
        let mut temp = String::new();
        if io::stdin().read_line(&mut temp).is_err() {
            println!("Invalid Input");
        }

        match temp.trim().parse::<i32>() {
            Ok(num) => {
                if num >= -1_000_000_000 && num <= 1_000_000_000 {
                    return num;
                }else {
                    println!("Please regard the range (-1,000,000,000 to 1,000,000,000)");
                } 
            },
            Err(_) => {
                println!("{} is not a valid number!!",temp.trim());
            }
        }
    };


}

fn interpreter(n: i32) -> String {

    let mut s = String::new();
    let mut n = n;

    if n < 0 {
        s.push_str("negative ");
        n = -n;
    }
    if n == 1_000_000_000 {
        s.push_str("One billion")
    }else if n >= 1000000 && n <= 999999999 {
        s.push_str(million(n).as_str());
    }else if n >= 100000 && n <= 999999 {
        s.push_str(hundred_thousand(n).as_str());
    }else if n >= 10000 && n <= 99999 {
        s.push_str(ten_thousand(n).as_str());
    }else if n >= 1000 && n <= 9999 {
        s.push_str(thousand(n).as_str());
    }else if n >= 100 && n <= 999 {
        s.push_str(hundred(n).as_str());
    }else if n < 100 {
        s.push_str(tees(n).as_str());
    }
    s
}

fn million(n: i32) -> String {
    let n0 = if n / 1_000_000 >= 10 {
        if n / 1_000_000 >= 100 && n / 1_000_000 <= 999 {
            hundred(n / 1_000_000)
        }else {
            tees(n / 1_000_000)
        }
    }else {
        ONES[(n / 1_000_000) as usize].to_string()
    };
    let n0 = format!("{n0} million");
    let ns = n % 1_000_000;
    if ns > 0 {
        let ts = if ns >= 100_000 {
            hundred_thousand(ns)
        }else if ns >= 10_000 {
            ten_thousand(ns)
        }else if ns >= 1_000 {
            thousand(ns)
        }else {
            inter(ns)
        };
        return format!("{n0} {ts}");
    }
    n0.to_string()
}

fn hundred_thousand(n: i32) -> String {
    let mut s1 = hundred(n / 1000);
    s1 = format!("{s1} thousand");
    if n % 1000 > 0 {
        s1 = format!("{s1} {}",inter(n % 1000));
    }
    s1
}

fn ten_thousand(n: i32) -> String {
    let s1 = tees(n / 1000);
    if n % 1000 > 0 {
        let s2 = inter(n % 1000);
        return format!("{s1} thousand {s2}");
    }else {
        return format!("{s1} thousand");
    }
}

fn thousand(n: i32) -> String {
    let n0 = ONES[(n / 1000) as usize];
    let n0 = format!("{n0} thousand");

    if n % 1000 > 100 {
        let n1 = hundred(n % 1000);
        return format!("{n0} {n1}");
    }else if n % 100 > 0 {
        let n1 = tees(n % 100);
        return format!("{n0} and {n1}");
    }
    n0
}

fn hundred(n: i32) -> String {
    let ts = ONES[(n / 100) as usize];
    let s1 = format!("{ts} hundred");
    if n % 100 > 0 {
        let s2 = tees(n % 100);
        return format!("{s1} and {s2}");
    }else {
        return s1
    }
}

fn tees(n: i32) -> String {
    if n < 10 {
        return ONES[n as usize].to_string();
    }
    if n < 20 {
        return TEENS[(n % 10) as usize].to_string();
    }
    let s = TENS[(n / 10) as usize];
    if n % 10 != 0 {
        let s2 = ONES[(n % 10) as usize];
        return format!("{s}-{s2}");
    }
    s.to_string()
}

fn inter(n: i32) -> String {
    if n >= 100 && n <= 999 {
        return hundred(n);
    }else if n < 100 {
        return format!("and {}", tees(n));
    }
    "".to_string()
}
 
