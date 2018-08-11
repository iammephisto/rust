// This program demonstrates rust function
// Written by iammephisto@github / VIM / Kali_WSL

fn main() {
    println!("Function runs another function that prints a value.");
    another_function(5, 6);
    println!("Function returns 5");
    let x = five();
    println!("The value of x is: {}", x);
    println!("Functions return x+1");
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32){
    println!("Another function with value {}, {}", x, y);
}
