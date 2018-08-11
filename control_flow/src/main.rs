fn main() {
    println!("Part one, if's:");
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    println!("Part 2: if expression within a let statement:");

    let condition = true;
    let number = if condition {5} 
        else {6};
    println!("The value of number is: {}", number);

    println!("Loops:");
    loop {
        println!("again!");
        break;
    }

    println!("While:");
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    
    println!("LAUNCH THE ROCKET!");

    println!("While again, should be For:");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    println!("This is a for loop (for each element in array)");
    for element in a.iter() {
        println!("The value is {}", element);
    }

    println!("Using a For loop with a range.");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!!!!");
}
