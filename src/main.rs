use crate::inventory::Inventory;

fn main() {
    let mut inventory = Inventory::new("inventory.json".to_string());

    // Load data from file
    match inventory.load_data() {
        Ok(_) => println!("Inventory data loaded successfully!"),
        Err(err) => println!("Error loading data: {}", err),
    }

    loop {
        println!("\n** Rustventory Menu **");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        let mut choice = String::new();
        std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
        choice.pop(); // Remove trailing newline

        match choice.parse::<u8>() {
            Ok(1) => {
                // Prompt user for new product details and call add_product
                let mut name = String::new();
                println!("Enter product name: ");
                std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
                name.pop();

                let mut description = String::new();
                println!("Enter product description: ");
                std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
                description.pop();

                let price: f64 = loop {
                    println!("Enter product price: ");
                    let mut input = String::new();
                    std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
                    input.pop();

                    match input.parse::<f64>() {
                        Ok(value) => break value,
                        Err(_) => println!("Invalid input. Please enter a number."),
                    }
                };

                let quantity: i32 = loop {
                    println!("Enter product quantity: ");
                    let mut input = String::new();
                    std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
                    input.pop();

                    match input.parse::<i32>()>() {
                        Ok(value) => break value,
                        Err(_) => println!("Invalid input. Please enter a number."),
                    }
                };

                let product = Product { name, description, price, quantity };
                inventory.add_product(product);
                println!("Product added successfully!");
            }
            Ok(2) => {
                // Prompt user for product name and call edit_product
                let mut name = String::new();
                println!("Enter product name to edit: ");
                std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
                name.pop();

                match inventory.edit_product(&name) {
                    Ok(_) => println!("Product edited successfully!"),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Ok(3) => {
                // Prompt user for product name and call delete_product
                let mut name = String::new();
                println!("Enter product name to delete: ");
                std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
                name.pop();

                match inventory.delete_product(&name) {
                    Ok(_) => println!("Product deleted successfully!"),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Ok(4) => {
                println!("{}", inventory.generate_report());
            }
            Ok(5) => {
                // Save data to file before exiting
                match inventory.save_data() {
                    Ok(_) => println!("Inventory data saved successfully!"),
                    Err(err) => println!("Error saving data: {}", err),
                }
                println!("Exiting Rustventory...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}

