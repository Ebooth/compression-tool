use std::fs;

use std::collections::HashMap;
fn main() {
    let content = read_file("tests/gutenberg.txt").expect("could not read the file");
    let counter = count_characters_frequency(&content);
    println!("{counter:?}");
}

fn count_characters_frequency(text: &str) -> HashMap<char, u64> {
    let mut counter = HashMap::new();
    text.chars().for_each(|c| {
        *counter.entry(c).or_insert(0) += 1;
    });
    return counter;
}

fn read_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
