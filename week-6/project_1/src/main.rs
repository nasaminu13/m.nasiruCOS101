use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    let P = "Poundo Yam/Edikaikon Soup";
    let F = "Fried Rice and Chicken";
    let A = "Amala and Ewedu Soup";
    let E = "Eba and Egusi Soup";
    let W = "White Rice and Stew";

    let P_price = 3200.0;
    let F_price = 3000.0;
    let A_price = 2500.0;
    let E_price = 2000.0;
    let W_price = 2500.0;

    println!("\nWill you like to see our menu?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    if input1.trim().eq_ignore_ascii_case("yes") {
        println!("{} is {}, input P to order", P, P_price);
        println!("{} is {}, input F to order", F, F_price);
        println!("{} is {}, input A to order", A, A_price);
        println!("{} is {}, input E to order", E, E_price);
        println!("{} is {}, input W to order", W, W_price);
        println!("Enter your orders one at a time.");

        println!("What would you like?");
        io::stdin().read_line(&mut input2).expect("Failed to read input");

        println!("How many plates will you like to order?");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let qty: f64 = input3.trim().parse().expect("Not a valid number");
        let mut total_cost = 0.0;

        if input2.trim() == "P" {
            total_cost = P_price * qty;
        } else if input2.trim() == "F" {
            total_cost = F_price * qty;
        } else if input2.trim() == "A" {
            total_cost = A_price * qty;
        } else if input2.trim() == "E" {
            total_cost = E_price * qty;
        } else if input2.trim() == "W" {
            total_cost = W_price * qty;
        } else {
            println!("Invalid selection.");
            return;
        }

        if total_cost > 10000.0 {
            let discount = 0.05 * total_cost;
            total_cost -= discount;
            println!("You qualify for a discount of 5%. Your total bill is {}", total_cost);
        } else {
            println!("Your total bill is {}", total_cost);
        }
    } else if input1.trim().eq_ignore_ascii_case("no") {
        println!("Okay, have a nice day!");
    } else {
        println!("Invalid response. Please restart.");
    }
}