use std::fs;
use std::path;

fn score_char(input_char: char) -> u32 {
    if !input_char.is_ascii_alphabetic() {
        return 0;
    }

    let unicode_code = input_char as u32;
    let offset = match input_char.is_ascii_uppercase() {
        // A (0x41) to Z (0x5A) (Scoring 27 to 52)
        true => 0x41 - 27,
        // a (0x61) to z (0x7A) (Scoring 1 to 26)
        false => 0x60,
    };
    let result = unicode_code - offset;
    result
}


fn main() {
    let backpack_file_path = path::Path::new("Day_03/input.txt");

    let backpack_file_path_str = backpack_file_path.display();
    println!("Reading following file: {}", backpack_file_path_str);

    let contents = fs::read_to_string(backpack_file_path)
        .expect("Could not read file: {backpack_file_path_str}");

    let parsed_backpack_file: u32 = contents
        .trim()
        .split('\n')
        .map(
            |backpack: &str| {
            
            // Split the backpack in half
            let splitted_backpack = backpack.split_at(backpack.len()/2);

            // Find the ASCII characters that are in both sides
            // from the backpack
            let mut test: Vec<char> = splitted_backpack.0
                .chars()
                .filter(|c| splitted_backpack.1.contains(*c))
                .collect();
            
            // Remove the duplicates
            test.sort_unstable();
            test.dedup();

            // Map the a-z A-z characters into the priorities
            let score: u32 = test.into_iter().map(score_char).sum();
            score
        }).sum();

    // First Answer
    println!("Sum of the backpack priorities: {}", parsed_backpack_file);
}
