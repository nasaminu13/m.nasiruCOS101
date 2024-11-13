// Project 1
use std::io;

fn main() {
	println!("\nQuadratic Equation Solver");

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Enter the coefficient of a");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:f64 = input1.trim().parse().expect("Not a valid number");

	println!("Enter the coefficient of b");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:f64 = input2.trim().parse().expect("Not a valid number");
	
	println!("Enter of coefficient of c");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let c:f64 = input3.trim().parse().expect("Not a valid Number");

	let discriminant = b*b - 4.0 * a * c;

	if discriminant > 0.0 {
		let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
		let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
	println!("The roots are real");
	
	println!("Root1 = {}",root1);
	println!("Root2 = {}",root2);
	
	}else if discriminant == 0.0 {
		let root = -b / (2.0 * a);
		println!("The roots are real and equal");
		println!("Root:{}",root);
	}else {
		let real_part = -b / (2.0 * a);
		let imaginary_part = (discriminant.abs().sqrt()) / (2.0 * a);
		println!("The roots are complex and different");
		println!("Root1 = {} + {}i", real_part, imaginary_part);
		println!("Root2 = {} - {}i", real_part, imaginary_part);
	}
}
