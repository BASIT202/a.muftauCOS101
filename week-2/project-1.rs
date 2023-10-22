fn main() {
	let principal:f64= 520_000_000.0;
	let time:f64= 5.0;
	let rate:f64= 10.0;
	let a:f64 = (1.0 + (rate / 100.0));
	let z:f64 = a.powf(time);
	let amount = principal * z;

	let compound_interest = amount - principal;

	println!("The compound interest is ₦{}", compound_interest);
}