fn main() {
    println!("");
    println!("---------------------------------");

    println!("Converting between Fahrenheit & Celsius");

    let from_f = 75;
    let to_c = f_to_c(from_f);
    println!("From {}F to {}C", from_f, to_c);

    let from_c = 5;
    let to_f = c_to_f(from_c);
    println!("From {}C to {}F", from_c, to_f);

    println!("");
    println!("---------------------------------");
    println!("Generating Fibonacci numbers");

    let mut last_number: u32 = 0;
    let mut number: u32 = 1;
    while number < 20 {
        let current_number = number;
        println!("Current number: {} & last number: {}", number, last_number);
        number = generate_fibonacci(number, last_number);

        last_number = current_number;
    }

    println!("");
    println!("---------------------------------");
    println!("Twelve Days of Christmas");
    for day in (1..13) {
        println!("");
        println!("On the {}th day of Christmas, my true love gave to me", day);

        let mut iter = day;

        while iter > 0 {
            count_twelve_days_of_christmas(iter);
            iter -= 1
        }
    }
} // end fn main

fn f_to_c(mut f: i32) -> i32 {
    f -= 32;
    f = f * 5 / 9;
    f
}

fn c_to_f(mut c: i32) -> i32 {
    c = c * 9 / 5;
    c += 32;
    c
}

fn generate_fibonacci(number: u32, last_number: u32) -> u32 {
    if number == 1 && last_number == 0 {
        number + 1
    } else {
        number + last_number
    }
}

fn count_twelve_days_of_christmas(day: u8) {
    if day == 1 {
        println!("A partridge in a pear tree");
    }
    if day == 2 {
        println!("Two turtle doves");
    }
    if day == 3 {
        println!("Three french hens");
    }
    if day == 4 {
        println!("Four calling birds");
    }
    if day == 5 {
        println!("FFFIIIIIVVVEEE GGGGOOOOLLLDD RRRIIINNNGGSSS");
    }
    if day == 6 {
        println!("Six geese a-laying");
    }
    if day == 7 {
        println!("Seven swans a-swimming");
    }
    if day == 8 {
        println!("Eight maids a-milking");
    }
    if day == 9 {
        println!("Nine ladies dancing");
    }
    if day == 10 {
        println!("Ten lords a-leeping");
    }
    if day == 11 {
        println!("Eleven pipers piping");
    }
    if day == 12 {
        println!("Twelve drummers drumming");
    }
}
