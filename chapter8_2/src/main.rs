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

    println!("Formatting motherfucking strings...");
    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");
    let aa = format!("{}-{}-{}", a1, a2, a3);
    println!("{}", aa);

    let hello = "Здравствуйте";
    //let answer = &hello[0];
    // Error because Rust doesn't return index of strings, but index of each byte of a character, З is 2 bytes, wouldn't make sense so Rust says NAY. 
    let answer2 = &hello[0..4];
    println!("Splitting a string vector bytes (DANGER): {}", answer2);
    // Not the best way to slice a string from the Rust pov, because it's prone to crashes. Use chars instead.

    let mut count2: u32 = 0;
    for c in hello.chars() {
        count2 += 1;
        println!("Chars function on a String (GOOD): {}", c);
    }

    let mut count: u32 = 0;
    for y in hello.bytes(){
        count += 1;
        println!("Printing Bytes: {}", y);
    }

    println!("There are {} chars.", count2);
    println!("There are {} bytes.", count);

    let calc: u32 = count/count2;
    println!("Therefore each char has {} bytes.", calc);

}
