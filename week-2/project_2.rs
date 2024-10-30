fn main() {
	let toshiba:f64 = 450000.0 * 2.0;
	let mac:f64 = 1500000.0;
	let hp:f64 = 750000.0 * 3.0;
	let dell:f64 = 2850000.0 * 3.0;
	let acer:f64 = 250000.0;

	// Sum and Average
	let s = toshiba + mac + hp + dell + acer;
	println!("Sum is {}", s);
	let q:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	let a = s / q ;
	println!("Average is {}", a);

}