fn main() {
	let p:f64 = 520_000_000.00; 
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	let ar = 1.0 + (r / 100.0);
	let am = ar.powf(t) * p;
	let ci = am - p;
	print!("The compound interest is ₦{:.2}", ci);
}
