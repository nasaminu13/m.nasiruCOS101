fn main() {
    let name = "Muhammad Nasiru";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Ekpe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);

    let department:&'static str = "Software Engineering";
    let school:&'static str = "School of science and Technology";
    println!("Department: {}, \nSchool: {}", department,school);
}
