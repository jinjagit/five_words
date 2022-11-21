use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
#[derive(Clone)]
struct WordPair {
    words: Vec<Vec<char>>,
    indices: Vec<usize>
}

fn main() {
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

    let word_pairs: Vec<WordPair> = find_word_pairs(word_list.clone());

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

fn find_word_pairs(word_list: Vec<Vec<char>>) -> Vec<WordPair> {
    let mut word_pairs: Vec<WordPair> = vec![];

    for (i, char_vec) in word_list.iter().enumerate() {
        if i != word_list.len() - 1 {
            for j in (i + 1)..(word_list.len() - 1) {
                let mut a = char_vec.clone();
                let mut b = word_list[j].clone();

                a.append(&mut b);

                // println!("---------------------------------");
                // println!("{:?}", a.clone());

                if vec_has_unique_elements(a.clone()) {
                    let word_pair = WordPair {
                        words: vec![char_vec.clone(), word_list[j].clone()],
                        indices: vec![i, j],
                    };

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

fn print_word_pairs(word_pairs: Vec<WordPair>) {
    for (_i, pair) in word_pairs.iter().enumerate() {
        println!("{:?}", pair);
    }
}