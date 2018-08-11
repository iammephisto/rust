fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string(); // "initial contents".to_string();
    println!("String::new : {}", s);

    let p = String::from("Test!");
    println!("String::from : {}", p);

    let mut q = String::from("I am ");
    q.push_str("Jean!");
    println!("push_str: {}", q);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("Concatenate with + : {}", s3);

    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");

    let aa = format!("{}-{}-{}", a1, a2, a3);

    println!("{}", aa);
}
