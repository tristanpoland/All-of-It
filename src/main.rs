use all_of_it::AllOfIt;
use std::io::{self, Write};

fn main() {
    println!("WARNING: This program will attempt to allocate ALL available memory.\nProceed at your own risk! Press Enter to continue...");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    match AllOfIt::new() {
        Some(all) => {
            println!("Success! Allocated {} bytes. Your system survived... for now.", all.capacity());
        }
        None => {
            println!("Failed to allocate memory. Your system is safe (or just stingy).");
        }
    }
}
