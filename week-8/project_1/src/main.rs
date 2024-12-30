use std::io;

fn main() {
    let mut department_list = vec![
        "Office_Administration",
        "Academic",
        "Lawyer",
        "Teacher",
    ];
    let mut office_administration_positions = vec![
        "Intern",
        "Administrator",
        "Senior Administrator",
        "Office Manager",
        "Director",
        "CEO",
    ];
    let mut academic_positions = vec![
        "Research Assistant",
        "PhD Candidate",
        "Post-Doc Researcher",
        "Senior Lecturer",
        "Dean",
    ];
    let mut lawyer_positions = vec![
        "Paralegal",
        "Junior Associate",
        "Associate",
        "Senior Associate 1-2",
        "Senior Associate 3-4",
        "Partner",
    ];
    let mut teacher_positions = vec![
        "Placement",
        "Classroom Teacher",
        "Senior Teacher",
        "Leading Teacher",
        "Deputy Principal",
        "Principal",
    ];

    let mut department = String::new();
    let mut work_experience = String::new();
    let mut position = String::new();

    println!("This is a Public Service APS Level Checker");
    println!("Enter your department:");
    std::io::stdin()
        .read_line(&mut department)
        .expect("Failed to read input");
    department = department.trim().to_string();

    if department == "Office_Administration" {
        println!("What is your position?");
        std::io::stdin()
            .read_line(&mut position)
            .expect("Failed to read input");
        position = position.trim().to_string();

        if position == "Intern" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 1.0 && work_experience <= 2.0 {
                println!("Your Public Service APS Level is APS 1-2");
            }
        } else if position == "Administrator" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 3.0 && work_experience <= 5.0 {
                println!("Your Public Service APS Level is APS 3-5");
            }
        } else if position == "Senior Administrator" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 5.0 && work_experience <= 8.0 {
                println!("Your Public Service APS Level is APS 5-8");
            }
        } else if position == "Office Manager" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 8.0 && work_experience <= 10.0 {
                println!("Your Public Service APS Level is EL1 8-10");
            }
        } else if position == "Director" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 10.0 && work_experience <= 13.0 {
                println!("Your Public Service APS Level is EL2 10-13");
            }
        } else if position == "CEO" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 13.0 {
                println!("Your Public Service APS Level is SES");
            }
        } else {
            println!("Invalid Input");
        }
    } 
    else if department == "Academic" {
        println!("What is your position?");
        std::io::stdin()
            .read_line(&mut position)
            .expect("Failed to read input");
        position = position.trim().to_string();

        if position == "Research Assistant" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 3.0 && work_experience <= 5.0 {
                println!("Your Public Service APS Level is APS 3-5");
            }
        } else if position == "PhD Candidate" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 5.0 && work_experience <= 8.0 {
                println!("Your Public Service APS Level is APS 5-8");
            }
        } else if position == "Post-Doc Researcher" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 8.0 && work_experience <= 10.0 {
                println!("Your Public Service APS Level is EL1 8-10");
            }
        } else if position == "Senior Lecturer" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 10.0 && work_experience <= 13.0 {
                println!("Your Public Service APS Level is EL2 10-13");
            }
        } else if position == "Dean" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 13.0 {
                println!("Your Public Service APS Level is SES");
            }
        } else {
            println!("Invalid Input");
        }
    } else if department == "Lawyer" {
        println!("What is your position?");
        std::io::stdin()
            .read_line(&mut position)
            .expect("Failed to read input");
        position = position.trim().to_string();

        if position == "Paralegal" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 1.0 && work_experience <= 2.0 {
                println!("Your Public Service APS Level is APS 1-2");
            }
        }
        else if position == "Junior Associate" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 3.0 && work_experience <= 5.0 {
                println!("Your Public Service APS Level is APS 3-5");
            }
        } else if position == "Associate" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 5.0 && work_experience <= 8.0 {
                println!("Your Public Service APS Level is APS 5-8");
            }
        } else if position == "Senior Associate 1-2" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 8.0 && work_experience <= 10.0 {
                println!("Your Public Service APS Level is EL1 8-10");
            }
        } else if position == "Senior Associate 3-4" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 10.0 && work_experience <= 13.0 {
                println!("Your Public Service APS Level is EL2 10-13");
            }
        } else if position == "Partner" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 13.0 {
                println!("Your Public Service APS Level is SES");
            }
        } else {
            println!("Invalid Input");
        }
    } else if department == "Teacher" {
        println!("What is your position?");
        std::io::stdin()
            .read_line(&mut position)
            .expect("Failed to read input");
        position = position.trim().to_string();

        if position == "Placement" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 1.0 && work_experience <= 2.0 {
                println!("Your Public Service APS Level is APS 1-2");
            }
        }
        else if position == "Classroom Teacher" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience >= 3.0 && work_experience <= 5.0 {
                println!("Your Public Service APS Level is APS 3-5");
            }
        } else if position == "Senior Teacher" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 5.0 && work_experience <= 8.0 {
                println!("Your Public Service APS Level is APS 5-8");
            }
        } else if position == "Leading Teacher" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 8.0 && work_experience <= 10.0 {
                println!("Your Public Service APS Level is EL1 8-10");
            }
        } else if position == "Deputy Principal" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 10.0 && work_experience <= 13.0 {
                println!("Your Public Service APS Level is EL2 10-13");
            }
        } else if position == "Principal" {
            println!("What is your work experience?");
            std::io::stdin()
                .read_line(&mut work_experience)
                .expect("Failed to read input");
            let work_experience: f64 = work_experience.trim().parse().expect("Invalid input");
            if work_experience > 13.0 {
                println!("Your Public Service APS Level is SES");
            }
        } else {
            println!("Invalid Input");
        }
    } 
        
     
    else {
        println!("Invalid Input");
    }
}
