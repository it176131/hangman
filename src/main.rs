use std::io;

fn main() {
    println!("Let's play hangman!");
    println!("Player 1, give me a category:");
    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");
    // Trime whitespace from `category`.
    let category = category.trim();
    println!("The category is \"{category}\"");
    let word = rpassword::prompt_password("Player 1, give me a word in the category:")
        .unwrap();
    println!("The word is \"{word}\"");
}
