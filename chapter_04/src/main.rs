fn main() {

    println!("References");

    let s1 = String::from("hello");

    // passing s1 in by reference
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    println!("");
    println!("-------------------------");
    println!("Mutable References");

    let mut s = String::from("hello");

    println!("s is: {}", s);
    change(&mut s);
    println!("s is: {}", s);

    println!("");
    println!("-------------------------");
    println!("Slice Type");

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    println!("first word: {}", word);

    let word = first_word(&my_string[0..4]);
    println!("first word: {}", word);

    let word = first_word(&my_string[..]);
    println!("first word: {}", word);

    let word = first_word(&my_string);
    println!("first word: {}", word);

}

// defining that s is passed in by reference
fn calculate_length(s: &String) -> usize {
    s.len()
}

// defining a mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// find the first word in a string (heap)
// returns a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}