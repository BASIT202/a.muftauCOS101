use std::io;

 fn main () {
    

    //The Incentive Calculator 
let mut age = String::new();
let mut experience = String::new();

println!("Enter Age");
io::stdin().read_line(&mut age).expect("Invalid input was used");
let age:f32 = age.trim().parse().expect("Invalid input");

println!("Are you experienced? Enter false or true");
io::stdin().read_line(&mut experience).expect("Invalid input");
let experience:bool = experience.trim().parse().expect("Invalid input");
    
    if experience {
        if age >= 40.0 {
       println!("Your annual incentive is 1,560,000");
    }
else if age >= 30.0 {
    println!("Your annual incentive is 1,480,000");
}

else if age < 28.0{
println!("Your annual incentive is 1,300,000");
}
   else {
       //ages 28 and 29 are not covered by the given brackets
       println!("Your annual incentive is 1,300,000");
   };
 }
 else {
    println!("Your annual incentive is 100,000");
 };

println!("Your experience status is: {}", experience);
println!("Your age is: {}", age);

}

