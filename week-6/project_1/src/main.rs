use std::io;

fn area(a: i32, b: i32, h:i32) {

    let area = h / 2 * ( a + b);

    println!("Area of trapezium = {}", area);
}

fn area_2(a: f64, b: f64) {
	let area_2 = 0.5 * ( a * b );

	println!("Area of rhombus = {}", area_2);
}

fn area_3(b: i32, h: i32) {
	let area_3 = b * h;

	println!("Area of parallelogram = {}", area_3);
}

fn area_4(l: i32) {
	let area_4 = 6 * l * l;

	println!("Area of cube = {}", area_4);
}


fn volume(r: f64, h: f64) {
	let volume = 3.14 * r * r * h;

	println!("Volume of cylinder = {}", volume);
}

fn main() {
	 println!("S/N");
	 println!("(1)Area of trapezium");
	 println!("(2)Area of rhombus");
	 println!("(3)Area of parallelogram");
	 println!("(4)Area of cube");
	 println!("(5)Volume of cylinder");

	 let mut inputx = String::new();
	 println!("Enter S/N of chosen calculation");
	 io::stdin().read_line(&mut inputx).expect("Failed to read input");
	 let function: f64 = inputx.trim().parse().expect("Not a valid S/N");

	if function == 1.0 {
	let mut input1 = String::new();
	println!("Enter input for parameter A: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let a: i32 = input1.trim().parse().expect("Invalid input");

	let mut input2 = String::new();
	println!("Enter input for parameter B: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b: i32 = input2.trim().parse().expect("Invalid input");

	let mut input3 = String::new();
	println!("Enter input for parameter H: ");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let h: i32 = input3.trim().parse().expect("Invalid input");

	area(a, b, h);
	}

	else if function == 2.0 {
	let mut input1 = String::new();
	println!("Enter input for parameter A: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let a: f64 = input1.trim().parse().expect("Invalid input");

	let mut input2 = String::new();
	println!("Enter input for parameter B: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b: f64 = input2.trim().parse().expect("Invalid input");

	area_2(a, b);
	}

	else if function == 3.0 {
	let mut input1 = String::new();
	println!("Enter input for parameter B: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let b: i32 = input1.trim().parse().expect("Invalid input");

	let mut input2 = String::new();
	println!("Enter input for parameter H: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let h: i32 = input2.trim().parse().expect("Invalid input");

	area_3(b, h);
	}	

	else if function == 4.0 {

	let mut input1 = String::new();
	println!("Enter input for parameter L: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let l: i32 = input1.trim().parse().expect("Invalid input");

	area_4(l);
	}

	else if function == 5.0 {
	let mut input1 = String::new();
	println!("Enter input for parameter R: ");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let r: f64 = input1.trim().parse().expect("Invalid input");

	let mut input2 = String::new();
	println!("Enter input for parameter H: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let h: f64 = input2.trim().parse().expect("Invalid input");

	volume(r, h);

	}	
}

