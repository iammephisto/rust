fn main() {
    // Working with structs
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    } // there is no ; here for some reason

    let mut user1 = User {
        email: String::from("test@email.com"),
        username: String::from("mephisto"),
        sign_in_count: 12,
        active: false,
    };

    println!("Email of user 1: {}", &user1.email);
    user1.email = String::from("wontwork@crash.com");
    println!("New value: {}", &user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("john"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    tuple_structs();

}

fn tuple_structs(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

/*fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}*/
