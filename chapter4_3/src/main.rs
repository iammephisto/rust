fn main() {
    println!("Slices part 1");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    println!("Part 2: Return a string slice");

    let word = first_word(&s[..]); // Slice the String before passing it for it to become a &str type
    println!("{}", word);
    
    println!("Now printing a literal (it's already a slice)");
    let second = "This is a literal";
    let word = first_word(&second);

    println!("{}", word);

    println!("Now slicing numbers arrays:");

    let numbers = [1, 2, 3, 4, 5];
    let slicednumbers = &numbers[1..3].len();

    println!("Length of numbers slice: {}", slicednumbers);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
