struct Laptops {
    brand: String,
    price:i32,
    quantity:i32,
}
impl Laptops {
    fn total_cost(&self, order_quantity:i32)->i32 {
        self.price*order_quantity
    }
}

fn main() {
    let art1 = Laptops{
        brand:String::from("HP"),
        price: 650_000,
        quantity: 10,
    };

    let art2 = Laptops {
        brand:String::from("IBM"),
        price: 755_000,
        quantity: 6,
    };

    let art3 = Laptops{
        brand:String::from("Toshiba"),
        price: 550_000,
        quantity: 10,
    };

    let art4 = Laptops{
        brand:String::from("Dell"),
        price: 850_000,
        quantity: 4,
    };

    let mut inventory:Vec<Laptops>=Vec::new();
    inventory.push(art1);
    inventory.push(art2);
    inventory.push(art3);
    inventory.push(art4);

    let total_order_quantity = 3;
    let mut total_cost = 0;

    for laptop in inventory.iter() {
        total_cost += laptop.total_cost(total_order_quantity);
    }
    println!("The cost for 3 laptops from each brand:{}",total_cost);
}