use std::io;
struct Employee {
    name: String,
    experience: u32,
}

fn main() {
    let mut employees = Vec::new();
    
    // Assuming we want to read input of 5 employees
    for _ in 0..5 {
        let mut name = String::new();
        let mut experience = String::new();
        
        
        println!("Enter developer's name:");
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();
        
        
        println!("Enter developer's years of experience:");
        io::stdin().read_line(&mut experience).expect("Failed to read line");
        let experience: u32 = experience.trim().parse().expect("Please enter a valid number");

        
        employees.push(Employee { name, experience });
    }

    
    let most_experienced = employees.iter().max_by(|a, b| a.experience.cmp(&b.experience));

    match most_experienced {
        Some(dev) => println!("The most experienced developer is: {:?} with {} years of experience.", dev.name, dev.experience),
        None => println!("No developers found."),
    }
}