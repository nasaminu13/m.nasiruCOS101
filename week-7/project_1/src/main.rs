 use std::io;
 fn main() {
    
    let mut input1 = String::new();
    let mut height_1 = String::new();
    let mut base_1 = String::new();
    let mut base_2 = String::new();
    let mut diagonal_1 = String::new();
    let mut diagonal_2 = String::new();
    let mut base = String::new();
    let mut altitude = String::new();
    let mut lenght_of_side = String::new();
    let mut radius = String::new();
    let mut height_2 = String::new();

    println!("Welcome to MTH101, what would you like to calculate? Start with capital letters!");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    if input1.trim() == "Area of Trapezium" {
        println!("What is the height of the Trapezium");
        io::stdin().read_line(&mut height_1).expect("Failed to read input");
        let height:f64 = height_1.trim().parse().expect("Failed to read input");

        println!("What is the value of base1?");
        io::stdin().read_line(&mut base_1).expect("Failed to read input");
        let base_1:f64 = base_1.trim().parse().expect("Failed to read input");

        println!("What is the value of base2?");
        io::stdin().read_line(&mut base_2).expect("Failed to read input");
        let base_2:f64 = base_2.trim().parse().expect("Failed to read input");

        let sum = base_1 + base_2;
        let Area_of_Trapezium:f64 = sum * height / 2.0;
        println!("Area of Trapezium = {}", Area_of_Trapezium);

    }
    else if input1.trim() =="Area of rhombus"{
        println!("What is the value of diagonal1");
        io::stdin().read_line(&mut diagonal_1).expect("Failed to read input");
        let diagonal_1:f64 = diagonal_1.trim().parse().expect("Failed to read input");

        println!("What is the value of diagonal2");
        io::stdin().read_line(&mut diagonal_2).expect("Failed to read input");
        let diagonal_2:f64 = diagonal_2.trim().parse().expect("Failed to read input");
        let Area_of_rhombus:f64 = 0.5 * diagonal_1 * diagonal_2;
        println!("Area of rhombus = {}",Area_of_rhombus);

    }
    else if input1.trim() =="Area of Parallelogram"{
        println!("What is the value of the base");
        io::stdin().read_line(&mut base).expect("Failed to read input");
        let base:f64 = base.trim().parse().expect("Failed to read input");

        println!("What is the value of the altitude");
        io::stdin().read_line(&mut altitude).expect("Failed to read input");
        let altitude:f64 = altitude.trim().parse().expect("Failed to read input");
        let Area_of_Parallelogram:f64 = base * altitude;
        println!("Area of Parallelogram = {}",Area_of_Parallelogram);
    } 
    else if input1.trim() == "Area of Cube"{
        println!("What is the lenght of the side");
        io::stdin().read_line(&mut lenght_of_side).expect("Failed to read input");
        let lenght_of_side:f64 = lenght_of_side.trim().parse().expect("Failed to read input");
        let Area_of_Cube:f64 = 6.0 * (lenght_of_side * lenght_of_side);
        println!("Area of Cube = {}",Area_of_Cube);
    }    
    else if input1.trim() =="Volume of Cylinder"{
        println!("What is the value of the radius");
        io::stdin().read_line(&mut radius).expect("Failed to read input");
        let radius:f64 = radius.trim().parse().expect("Failed to read input");

         println!("What is the height of the cylinder");
        io::stdin().read_line(&mut height_2).expect("Failed to read input");
        let height_2:f64 = height_2.trim().parse().expect("Failed to read input");
        let Volume_of_cylinder:f64 = 3.142 * height_2 * (radius * radius) ;
        println!("Volume of the cylinder = {}",Volume_of_cylinder);
    } 
    else {
        println!("Invalid Input");
    }
}