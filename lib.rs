use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, stdin};
//use std::io::{self, BufRead};
//use std::io::stdin;
use serde_json::{from_str, to_string};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}

pub struct Inventory {
    products: HashMap<String, Product>,
    data_file: String,
}

impl Inventory {
    pub fn new(data_file: String) -> Self {
        Inventory {
            products: HashMap::new(),
            data_file,
        }
    }

    pub fn load_data(&mut self) -> Result<(), String> {
        let mut file = File::open(&self.data_file).map_err(|err| format!("Error opening file: {}", err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|err| format!("Error reading file: {}", err))?;

        if !contents.is_empty() {
            let products: Vec<Product> = serde_json::from_str(&contents).map_err(|err| format!("Error deserializing data: {}", err))?;
            for product in products {
                self.products.insert(product.name.clone(), product);
            }
        }

        Ok(())
    }

    pub fn save_data(&self) -> Result<(), String> {
        let serialized_data = serde_json::to_string(&self.products).map_err(|err| format!("Error serializing data: {}", err))?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.data_file)
            .map_err(|err| format!("Error opening file for writing: {}", err))?;
        file.write_all(serialized_data.as_bytes()).map_err(|err| format!("Error writing to file: {}", err))?;

        Ok(())
    }

    pub fn add_product(&mut self, product: Product) {
        if self.products.contains_key(&product.name) {
            println!("Product with name '{}' already exists!", product.name);
            return;
        }

        self.products.insert(product.name.clone(), product);
        self.save_data().expect("Error saving data");
    }

    pub fn edit_product(&mut self, name: &str) -> Result<(), String> {
        let mut product = match self.products.get_mut(name) {
            Some(p) => p,
            None => return Err(format!("Product '{}' not found", name)),
        };

        println!("Enter new values (or leave blank to keep unchanged):");
        let mut new_name = String::new();
        println!("New name: ");
        std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
        new_name.pop();

        let mut new_description = String::new();
        println!("New description: ");
        std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
        new_description.pop();

        let mut new_price: Option<f64> = None;
        loop {
            println!("New price (leave blank to keep unchanged): ");
            let mut input = String::new();
            std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
            input.pop();

            if input.is_empty() {
                break;
            }

            match input.parse::<f64>() {
                Ok(value) => {
                    new_price = Some(value);
                    break;
                }
                Err(_) => println!("Invalid input. Please enter a number."),
            }
        }

        let mut new_quantity: Option<i32> = None;
        loop {
            println!("New quantity (leave blank to keep unchanged): ");
            let mut input = String::new();
            std::io::read_line(&mut std::io::stdin()).expect("Failed to read input");
            input.pop();

            if input.is_empty() {
                
                break;
            }

            match input.parse::<i32>() {
                Ok(value) => {
                    new_quantity = Some(value);
                    break;
                }
                Err(_) => println!("Invalid input. Please enter a number."),
            }
        }

        // Update product details only if new values are provided
        if !new_name.is_empty() {
            product.name = new_name;
        }
        if !new_description.is_empty() {
            product.description = new_description;
        }
        if new_price.is_some() {
            product.price = new_price.unwrap();
        }
        if new_quantity.is_some() {
            product.quantity = new_quantity.unwrap();
        }

        self.save_data().expect("Error saving data");
        Ok(())
    }

    pub fn remove_product(&mut self, name: &str) -> Result<(), String> {
        if self.products.remove(name).is_none() {
            return Err(format!("Product '{}' not found", name));
        }

        self.save_data().expect("Error saving data");
        Ok(())
    }

    pub fn list_products(&self) {
        if self.products.is_empty() {
            println!("No products found!");
            return;
        }

        println!("| Name | Description | Price | Quantity |");
        println!("|---|---|---|---|");
        for product in self.products.values() {
            println!(
                "| {} | {} | {:.2} | {} |",
                product.name, product.description, product.price, product.quantity
            );
        }
    }
}
