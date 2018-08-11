fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // appends at the end of the string
    println!("{}", s);
    println!("Now doing clone:");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    println!("Next step:");

    let s3 = gives_ownership();
    println!("{}", s3);
    let s4 = String::from("hello");
    println!("{}", s4);
    let s5 = takes_and_gives_back(s4);
    println!("{}", s5);

    let s6 = String::from("hello");
    let (s7, len) = calculate_length(s6);

    println!("The length of '{}' is {}.", s7, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len returns the lenth of a string
    (s, length)
}
