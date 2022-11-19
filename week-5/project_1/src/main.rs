use std::io;

fn main() 
{
	let mut rootA: f32 = 0.0;
	let mut rootB: f32 = 0.0;

	let mut disc:f32 = 0.0;
	let mut realp:f32 = 0.0;
    let mut imagp:f32 = 0.0;

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("a:");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:f32 = input1.trim().parse().expect("Not a valid number");

	println!("b:");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:f32 = input2.trim().parse().expect("Not a valid number");
    
    println!("c:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    disc = b * b - 4.0 * a * c;

    if disc > 0.0
    {
    	println!("Roots are real and distinct");
    	disc = disc.abs();
    	rootA = (-b + disc.sqrt()) / (2.0 * a);
        rootB = (-b - disc.sqrt()) / (2.0 * a);
    	println!("Root1 = {}  +i {}", realp, imagp);
        println!("Root2 = {}  -i {}", realp, imagp);
    }

    if disc == 0.0
    {
    	println!("Roots are real");
    	disc = disc.abs();
    	rootA = -b / (2.0 * a);
        rootB = rootA;
    	println!("Root1 = {}  +i {}", realp, imagp);
        println!("Root2 = {}  -i {}", realp, imagp);
    }

    if disc < 0.0
    {
    	println!("Roots are imaginary");
    	disc = disc.abs();
    	realp = -b / (2.0 * a);
        imagp = disc.sqrt() / (2.0 * a);
    	println!("Root1 = {}  +i {}", realp, imagp);
        println!("Root2 = {}  -i {}", realp, imagp);
    }

    let mut root_a: f32 =  ( - b + disc.sqrt()) / 2.0 * a;
    rootA = rootA.sqrt();
    {
    	println!("Root of quadratic equation: {}", rootA);
    }
    let mut root_b: f32 = ( - b - disc.sqrt()) / 2.0 * a;
	rootB = rootB.sqrt();
	{
		println!("Root of quadratic equations: {}", rootB);
	}
}





