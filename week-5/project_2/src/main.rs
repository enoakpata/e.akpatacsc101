use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEnter experience (experience is 1 and inexperience is 0): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience: i32 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if experience == 1 && age >= 40
     {
    	println!("Incentive is 1560000.0");
    }
    else if experience == 1 &&  age >= 30 && age < 40
     {
    	println!("Incentive is 1480000.0");
    }
    else if experience == 1 && age < 28 
    {
    	println!("Incentive is 1300000.0 per month");
    }
    else if experience == 0
    {
    	println!("Incentive is 100000.0");
    }
}

