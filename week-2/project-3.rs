fn main() {
	let cost:f64= 210_000.0;
	let time:f64= 3.0;
	let rate:f64= 5.0;
	let a:f64 = (1.0 - (rate / 100.0));
	let b:f64 = a.powf(time);
	let amount = cost * b;



	println!("The new value of the television is ₦{}", amount);
	
}