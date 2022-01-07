fn main() {
    println!("Hello, world!");

    another_function();

    my_function(290);

    print_labeled_measurement(-23, 'F');
    let five = five();
    println!("Five is five: {}", five);
}

fn another_function() {
    println!("Another function.");
}

fn my_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}
