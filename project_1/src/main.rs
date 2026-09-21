use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

// Getting a,b and c

println!("Enter a");
io::stdin().read_line(&mut a).expect("Not a valid input");
let a:f32 = a.trim().parse().expect("Invalid input");

println!("Enter b");
io::stdin().read_line(&mut b).expect("Invalid input");
let b:f32 = b.trim().parse().expect("Invalid input");

println!("Enter c");
io::stdin().read_line(&mut c).expect("Invalid input");
let c :f32 = c.trim().parse().expect("Invalid input");

// Calculating the discriminant

let d:f32 = b*b - 4.0*a*c;

let root1:f32 = (-b + d.sqrt())/2.0*a;
let root2:f32 = (-b - d.sqrt())/2.0*a;


println!("The first root is equal to {} " ,root1);
println!("The second root is equal to {} " ,root2);

}


