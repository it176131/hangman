use std::io;

fn main() {
    println!("Let's play hangman!");
    println!("Player 1, give me a category:");
    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");
    println!("The category is \"{category}\"");
}
