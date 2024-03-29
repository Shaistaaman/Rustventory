use std::collections::HashMap;


#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn new(name: String, description: String, price: f64, quantity: i32) -> Result<Self, String> {
        if name.is_empty() {
            Err("Product name cannot be empty".to_string())
        } else if price <= 0.0 {
            Err("Price must be positive".to_string())
        } else if quantity < 0 {
            Err("Quantity cannot be negative".to_string())
        } else {
            Ok(Product {
                name,
                description,
                price,
                quantity,
            })
        }
    }
}

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    // Hardcoded credentials (replace with secure storage later)
    let valid_username = "admin";
    let valid_password = "secret";

        // Authentication loop
        loop {
            println!("Enter username:");
            username.clear();
            std::io::stdin().read_line(&mut username).unwrap();
            username = username.trim().parse::<String>().unwrap();
            
           

            println!("Enter password:");
            password.clear();
            std::io::stdin().read_line(&mut password).unwrap();
            password = password.trim().parse::<String>().unwrap();
            


            // Corrected comparison
            if username == valid_username && password == valid_password {
                break;
            } else {
                println!("Invalid credentials. Try again.");
            }
        }
    
    println!("--------------------");
    println!("Welcome!");
    println!("--------------------");

    let mut inventory: HashMap<String, Product> = HashMap::new();

    // Main menu loop
    loop {
        println!("\nMenu:");
        println!("1. Add product");
        println!("2. Edit product");
        println!("3. Delete product");
        println!("4. Generate report");
        println!("5. Exit");

        let mut choice = String::new();

        choice.clear();
        std::io::stdin().read_line(&mut choice).unwrap();
        choice = choice.trim().parse::<String>().unwrap();
            
        match choice.parse::<u8>() {
            Ok(1) => add_product(&mut inventory),
            Ok(2) => edit_product(&mut inventory),
            Ok(3) => delete_product(&mut inventory),
            Ok(4) => generate_report(&inventory),
            Ok(5) => {
                println!("Exiting program...");
                break;
            },
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn add_product(inventory: &mut HashMap<String, Product>) {
    let mut name = String::new();
    let mut description = String::new();
    let mut price: f64 = 0.0;
    let mut quantity: i32 = 0;

    println!("Enter product name:");
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().parse::<String>().unwrap();
    
    println!("Enter product description:");
    std::io::stdin().read_line(&mut description).unwrap();
    description = description.trim().parse::<String>().unwrap();
    

    loop {
        println!("Enter product price:");
        let mut price_input = String::new();
        std::io::stdin().read_line(&mut price_input).unwrap();
        price_input = price_input.trim().parse::<String>().unwrap();

        match price_input.parse::<f64>() {
            Ok(p) => {
                price = p;
                break;
            },
            Err(_) => println!("Invalid price. Please enter a number."),
        }
    }

    loop {
        println!("Enter product quantity:");
        let mut quantity_input = String::new();
        std::io::stdin().read_line(&mut quantity_input).unwrap();
        quantity_input = quantity_input.trim().parse::<String>().unwrap();

        match quantity_input.parse::<i32>() {
            Ok(q) => {
                quantity = q;
                break;
            },
            Err(_) => println!("Invalid quantity. Please enter a whole number."),
        }
    }

    match Product::new(name, description, price, quantity) {
        Ok(product) => {
            inventory.insert(product.name.clone(), product);
            println!("Product added successfully!");
        },
        Err(err_msg) => println!("{}", err_msg),
    }
}

fn edit_product(inventory: &mut HashMap<String, Product>) {
    let mut product_name = String::new();

    println!("Enter the name of the product to edit:");
    std::io::stdin().read_line(&mut product_name).unwrap();
    product_name = product_name.trim().parse::<String>().unwrap();

    if !inventory.contains_key(&product_name) {
        println!("Product not found.");
        return;
    }

    let mut choice = String::new();
    println!("What do you want to edit?");
    println!("1. Description");
    println!("2. Price");
    println!("3. Quantity");

    std::io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().parse::<String>().unwrap();

    
    let mut product = inventory.get_mut(&product_name).unwrap();

    match choice.parse::<u8>() {
        Ok(1) => {
            let mut new_description = String::new();
            println!("Enter new description:");
            std::io::stdin().read_line(&mut new_description).unwrap();
            new_description = new_description.trim().parse::<String>().unwrap();

            product.description = new_description;
            println!("Description updated successfully!");
        },
        Ok(2) => {
            loop {
                let mut new_price = String::new();
                println!("Enter new price:");
                std::io::stdin().read_line(&mut new_price).unwrap();
                new_price = new_price.trim().parse::<String>().unwrap();


                match new_price.parse::<f64>() {
                    Ok(p) => {
                        product.price = p;
                        break;
                    },
                    Err(_) => println!("Invalid price. Please enter a number."),
                }
            }
            println!("Price updated successfully!");
        },
        Ok(3) => {
            loop {
                let mut new_quantity = String::new();
                println!("Enter new quantity:");
                std::io::stdin().read_line(&mut new_quantity).unwrap();
                new_quantity = new_quantity.trim().parse::<String>().unwrap();

                match new_quantity.parse::<i32>() {
                    Ok(q) => {
                        product.quantity = q;
                        break;
                    },
                    Err(_) => println!("Invalid quantity. Please enter a whole number."),
                }
            }
            println!("Quantity updated successfully!");
        },
        _ => println!("Invalid choice."),
    }
}

fn delete_product(inventory: &mut HashMap<String, Product>) {
    let mut product_name = String::new();

    println!("Enter the name of the product to delete:");
    std::io::stdin().read_line(&mut product_name).unwrap();
    product_name = product_name.trim().parse::<String>().unwrap();

    
    if !inventory.contains_key(&product_name) {
        println!("Product not found.");
        return;
    }

    inventory.remove(&product_name);
    println!("Product deleted successfully!");
}

fn generate_report(inventory: &HashMap<String, Product>) {
    println!("Inventory Report:");
    println!("--------------------");
    println!("| Name     | Description                 | Price  | Quantity |");
    println!("|----------|-----------------------------|-------|---------|");

    for (name, product) in inventory.iter() {
        println!(
            "| {:<10} | {:<25} | {:.2} | {:>5} |",
            name, product.description, product.price, product.quantity
        );
    }

    println!("--------------------");
}
