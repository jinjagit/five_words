use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;

fn main() {
    println!("calulating...");
    println!();
    
    let start = SystemTime::now();

    // First, we read the word_list file and add only words we are interested in to a Vec.
    // Words must be 5 letters, with no other character types, and no duplicate characters.

    let mut word_list: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines("./words.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let word = ip.to_lowercase();

                // Check word is 5 characters, and all characters are letters
                if word.chars().count() == 5 && word.chars().all(|c| matches!(c, 'a'..='z')) {
                    let char_vec: Vec<char> = word.chars().collect();
                    // Check word does not contain duplicate characters
                    if vec_has_unique_elements(char_vec.clone()) {
                        word_list.push(char_vec);
                    }
                }
            }
        }
    }

    let word_pairs: Vec<Vec<Vec<char>>> = find_word_pairs(word_list.clone());

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    print_word_list(word_list.clone());
    println!();
    // print_word_pairs(word_pairs.clone());
    println!("{:?}", word_pairs[13].clone());
    println!();
    println!("Total runtime: {} ms", duration.as_millis());
    println!("word list length: {:?}", word_list.len());
    println!("word pairs length: {:?}", word_pairs.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn vec_has_unique_elements(mut char_vec: Vec<char>) -> bool {
    let mut char_vec_copy = char_vec.clone();

    char_vec_copy.sort();
    
    char_vec.sort();
    char_vec.dedup();

    if char_vec == char_vec_copy {
        return true;
    }

    false
}

fn vecs_no_dups(vec_a: Vec<char>, vec_b: Vec<char>)-> bool {
    for i in 0..vec_a.len() {
        for j in 0..vec_b.len() {
            if vec_a[i] == vec_b[j] {
                return false
            }
        }
    }

    true
}

fn find_word_pairs(word_list: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut word_pairs: Vec<Vec<Vec<char>>> = vec![];

    for i in 0..(word_list.len() - 1) {
        if i != word_list.len() - 1 {
            for j in (i + 1)..(word_list.len() - 1) {
                // println!("---------------------------------");
                // println!("{:?}", a.clone());

                if vecs_no_dups(word_list[i].clone(), word_list[j].clone()) {
                    let word_pair: Vec<Vec<char>> = vec![word_list[i].clone(), word_list[j].clone()];

                    // println!("{:?}", word_pair);

                    word_pairs.push(word_pair);
                }
            }

            // println!("i = {:?}", i);
        }
    }

    word_pairs
}

fn print_word_list(word_list: Vec<Vec<char>>) {
    for (_i, char_vec) in word_list.iter().enumerate() {
        println!("{:?}", char_vec);
    }
}

fn print_word_pairs(word_pairs: Vec<Vec<Vec<char>>>) {
    for (_i, pair) in word_pairs.iter().enumerate() {
        println!("{:?}", pair);
    }
}