use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    // First, we read the word_list file and add only words we are interested in to a Vec.
    // Words must be 5 letters, with no other character types, and no duplicate characters.

    let mut word_list: Vec<String> = vec![];

    if let Ok(lines) = read_lines("./words.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let word = ip.to_lowercase();

                // Check word is 5 characters, and all characters are letters
                if word.chars().count() == 5 && word.chars().all(|c| matches!(c, 'a'..='z')) {
                    // Check word does not contain duplicate characters
                    if string_has_unique_chars(&word) {
                        word_list.push(word);
                    }
                }
            }
        }
    }

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    println!("Total runtime: {} ms", duration.as_millis());
    println!();
    // println!("word_list: {:?}", word_list);
    println!("word list length: {:?}", word_list.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_has_unique_chars(string: &String) -> bool {
    let mut char_vec: Vec<char> = string.chars().collect();
    let char_vec_copy = char_vec.clone();
    char_vec.dedup();

    if char_vec == char_vec_copy {
        return true;
    }

    false
}
