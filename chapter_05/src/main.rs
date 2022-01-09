fn main() {
    println!("Hello, world!");
    let user1 = build_user(String::from("myEmail@email.com"), String::from("my_username"));

    println!("My username: {}", user1.username);
    println!("Email address [{}] has logged in {} time(s)", user1.email, user1.sign_in_count);

    if user1.active == true {
        println!("User is active");
    }

    // notice here that since the fields that do not implement the Copy trait are supplied, user3 & user1 both exist
    let user3 = User {
        email: String::from("user3@email.com"),
        username: String::from("user3_username"),
        ..user1
    };

    println!("My username: {}", user3.username);
    println!("Email address [{}] has logged in {} time(s)", user3.email, user3.sign_in_count);

    if user3.active == true {
        println!("User is active");
    }


    // shorthand struct copying
    let user2 = User {
        email: String::from("my_other_email@email.com"),
        ..user1
    };

    // in the above, user1 no longer exists because some of the fields do not implement the Copy trait

    println!("My username: {}", user2.username);
    println!("Email address [{}] has logged in {} time(s)", user2.email, user2.sign_in_count);

    if user2.active == true {
        println!("User is active");
    }

    let user4 = build_user_shorthand(String::from("user4@email.com"), String::from("user4"));

    println!("My username: {}", user4.username);
    println!("Email address [{}] has logged in {} time(s)", user4.email, user4.sign_in_count);

    if user4.active == true {
        println!("User is active");
    }

    let red = Color(255, 0, 0);
    println!("Red RGB: R{} G{} B{}", red.0, red.1, red.2);

}

fn build_user(email: String, username: String) -> User {

    User {
        email: email,
        active: true,
        sign_in_count: 1,
        username: username,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {

    // because the parameters and struct fields have the same names, we can use the shorthand to save some typing
    User {
        email,
        active: true,
        sign_in_count: 1,
        username,
    }
}

struct Color (
    i32, i32, i32
);

struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}
