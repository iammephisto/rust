fn main() {
    let s2 = change();
    println!("The value of s2 is: {}", s2);
}

fn change() -> String {
    let mut result = String::from("hello");
    result.push_str(", world");
    result
}
