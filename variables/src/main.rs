fn main() {
 	let mut x = 5;
	println!("The value of x is {}", x);
	x = 7;
	println!("The value of X is now {}", x);

	println!("Next: Spaces length. Shadowing.");
	let spaces = "     ";
	let spaces = spaces.len();
	println!("Spaces: {}", spaces);
}
