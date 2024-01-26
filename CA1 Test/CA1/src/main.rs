// Rust program to diagnose patients


use std::io;

fn main() {
     
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();
     let mut input4 = String::new();
     let mut input5 = String::new();
     let mut input6 = String::new();
     let mut input7 = String::new();
     let mut input8 = String::new();

     let amount_of_alzheimer:i32 = 1_200_000;
     let amount_of_arrhythmia:i32 = 550_000;
     let amount_of_chronic_kidney_disease:i32 = 1_500_000;
     let amount_of_diabetes:i32 = 800_000;
     let amount_of_arthritis:i32 = 450_000;

     let mut no_diagnosis:i32 = 0;

      let medical_diagnosis = " Alzheimer : ₦1,200,000 (alzheimer)\n
    Arrhythmia: ₦550,000 (arrhythmia) \n
    Chronic Kidney Disease: ₦1,500,000 (chronic_kidney_disease) \n
    Diabetes: ₦800,000 (diabetes) \n
    Arthritis: ₦450,000 (arthritis)";



     println!("Name: ");
     io::stdin().read_line(&mut input1).expect("invalid string");
     let name = input1.trim();

     println!("Date of birth: ");
     io::stdin().read_line(&mut input2).expect("invalid string");
     let date of birth = input2.trim();

     println!("Email address: ");
     io::stdin().read_line(&mut input3).expect("invalid string");
     let email address = input3.trim();

     println!("Phone Number: ");
     io::stdin().read_line(&mut input4).expect("invalid string");
     let phone number = input4.trim();

     println!("Number of  siblings: ");
     io::stdin().read_line(&mut input5).expect("invalid string");
     let number of siblings:i32 = input5.trim().parse().expect("invalid number");

     println!("Number of children: ");
     io:stdin().read_line(&mut input6).expect("invalid string");
     let number of children:i32 = input6.trim().parse().expect("invalid number");

     println!("Medical diagnosis: ");
     io::stdin().read_line(&mut input7).expect("invalid string");
     let medical diagnosis = input7.trim();

     println!("Village of residence: ");
     io::stdin().read_line(&mut input8).expect("invalid string");
     let village of residence = input8.trim();

     if medical diagnosis.to_lowercase() 







}