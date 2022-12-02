use std::fs;
use std::path;

fn main() {
    let elves_calories_file_path = path::Path::new("Day_01/input.txt");

    let elves_file_path_str = elves_calories_file_path.display();
    println!("Reading following file: {}", elves_file_path_str);

    let contents = fs::read_to_string(elves_calories_file_path)
        .expect("Could not read file: {elves_file_path_str}");

    // Parse the contents of the string
    // and sum them up as integeres
    //
    // Format:
    // List of numbers seperated by newline
    // A new list is introduced by a seperate new line
    //
    // Example:
    // 1
    // 2
    //
    // 3
    // 4
    // Would be parsed to: [3, 7]
    let parsed_elv_list: Vec<i32> = contents
        .trim()
        .split("\n\n")
        .map(
            |elv_calories: &str| elv_calories
            .split('\n')
            .map(|elv_calory: &str| elv_calory
                .parse::<i32>().expect("Could not parse number!"))
            .sum()
        )
        .collect();
    
    // We need to find the index of the highest value from the list
    // E.g: a = [1, 3]
    //
    // Then the second list (a[1]) would be the target!
    let index_of_elf_with_max_calories = parsed_elv_list
        .iter()
        .enumerate()
        .max_by(|(_, x1), (_, x2)| x1.cmp(x2)).map(|(key, _)| key)
        .expect("Could not find the maximum value of the list");
    
    println!("Index of Elv: {}", index_of_elf_with_max_calories);
    println!("Max Calories: {}", parsed_elv_list[index_of_elf_with_max_calories]);
}
