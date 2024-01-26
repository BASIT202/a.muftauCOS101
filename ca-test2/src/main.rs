use std::fs::File;
use std::io::{self, Write};

    struct company {
        company name: String,
        date: i32,
        assets: i32,
        liabilities: i32,
    }


        fn login()
        println!("Enter username:");
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("username failed");

        println!("Enter password:");
        let mut password =  String::new();
        io::stdin().read_line(&mut password).expect("password failed");

        let usernames = vec!["CNP", "CBP", "DSRP", "FMNP", "NNP", "UNP", "HNP", "NBP"];

        let passwords = vec!["123", "abc", "ab2", "12k", "tt7", "109", "lik", "amg"];
        
        let usernames = username.trim().len()>=3 && username.trim().len()<=8

        let passwords = password.trim().len() 


        fn main() {
            if login() {
                let companies = vec![
                    company {
                        company name: String::from("cadbury"),
                        date: 1965,
                        assets: 15_000_000
                        liabilities: 5_500_000
                    }
                     company {
                        company name: String::from("champion"),
                        date: 1974,
                        assets: 25_000_000
                        liabilities: 8_000_000
                    }
                     company {
                        company name: String::from("dangote"),
                        date: 1970,
                        assets: 18_000_000
                        liabilities: 10_000_000
                    }
                     company {
                        company name: String::from("flour"),
                        date: 1960,
                        assets: 32_000_000
                        liabilities: 4_000_000
                    }
                     company {
                        company name: String::from("nestle"),
                        date: 1961,
                        assets: 8_000_000
                        liabilities: 1_500_000
                    }
                     company {
                        company name: String::from("unilever"),
                        date: 1923,
                        assets: 37_000_000
                        liabilities: 11_000_000
                    }
                     company {
                        company name: String::from("honeywell"),
                        date: 1906,
                        assets: 34_000_000
                        liabilities: 9_000_000
                    }
                     company {
                        company name: String::from("nigerian"),
                        date: 1946,
                        assets: 30_000_000
                        liabilities: 12_000_000
                    }
                ]
              let mut data_file = create::file("data.txt").expect("file creation failed");
            }
        }

    




    }