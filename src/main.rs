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
    let len = word.chars().count();
    println!("There are {len} letters in the word.");
    let mut incorrect: u8 = 0;
    println!("Player 2, guess a letter:");
    let mut letter = String::new();
    io::stdin()
        .read_line(&mut letter)
        .expect("Failed to read line");
    let letter = letter.trim();
    println!("The letter is \"{letter}\"");
    match word.contains(letter) {
        true => println!("The word contains the letter {letter}."),
        false => {
            println!("The word does _not_ contain the letter {letter}.");
            incorrect += 1;
            println!("Total incorrect: {incorrect}");
        },
    }
}
