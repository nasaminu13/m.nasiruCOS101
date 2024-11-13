use std::io;

fn main(){

	println!("Employee Annual Incentive");

	let mut experience = String::new();
	let mut age = String::new();

	println!("Enter the experience of employee in years");
	io::stdin().read_line(&mut experience).expect("Invalid string");
	let experience:u32 = experience.trim().parse().expect("Invalid number");

	println!("Enter the age of employee in years");
	io::stdin().read_line(&mut age).expect("Invalid string");
	let age:u32 = age.trim().parse().expect("Invalid number");

	let incentive1 = 1560000.0;
	let incentive2 = 1480000.0;
	let incentive3 = 1300000.0;
	let incentive4 = 100000.0;

	if experience >= 5 && age >= 40{
		println!("Your incentive per month is: {}",incentive1);
	}else if experience >= 5 && age >= 30 && age <40{
		println!("Your incentive per month is: {}",incentive2);
	}else if experience >= 5 && age < 28{
		println!("Your incentive per month is: {}",incentive3);
	}else if experience < 5{
		println!("Your incentive per month is: {}",incentive4);
	}
}
