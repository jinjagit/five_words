use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;

fn main() {
    println!("calculating...");
    println!();
    
    let start = SystemTime::now();

    // Read the word_list file and add only suitable words to word_list Vec.
    // Words must be 5 letters, with no other character types, and no duplicate characters.
    // 80 ms

    let mut word_list: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines("./words.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let word = ip.to_lowercase();

                // Check word is 5 characters, and all characters are letters
                if word.chars().count() == 5 && word.chars().all(|c| matches!(c, 'a'..='z')) {
                    let char_vec: Vec<char> = word.chars().collect();
                    // Check word does not contain duplicate characters
                    if vec_has_no_dups(char_vec.clone()) {
                        word_list.push(char_vec);
                    }
                }
            }
        }
    }

    // Find pairs of words with no shared characters
    // 10 s

    let word_pairs: Vec<Vec<char>> = find_word_pairs(word_list.clone());

    let word_quartets: Vec<Vec<char>> = find_word_pairs(word_pairs.clone());

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    print_word_list(word_list.clone());
    println!();
    // print_word_pairs(word_pairs.clone());
    println!("{:?}", word_pairs[13].clone());
    println!();

    println!("{:?}", word_quartets[13].clone());
    println!();
    println!("Total runtime: {} ms", duration.as_millis());
    println!("word list length: {:?}", word_list.len());
    println!("word pairs length: {:?}", word_pairs.len());
    println!("word quartets length: {:?}", word_quartets.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn vec_has_no_dups(vec: Vec<char>)-> bool {
    let len: usize = vec.len();

    for i in 0..len {
        for j in (i + 1)..len {
            if vec[i] == vec[j] {
                return false
            }
        }
    }

    true
}

fn vecs_have_no_dups(vec_a: Vec<char>, vec_b: Vec<char>)-> bool {
    for i in 0..vec_a.len() {
        for j in 0..vec_b.len() {
            if vec_a[i] == vec_b[j] {
                return false
            }
        }
    }

    true
}

fn find_word_pairs(word_list: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut word_pairs: Vec<Vec<char>> = vec![];
    let word_list_len = word_list.len();

    for i in 0..word_list_len {
        if i != word_list_len - 1 {
            for j in (i + 1)..word_list_len {
                if vecs_have_no_dups(word_list[i].clone(), word_list[j].clone()) {
                    word_pairs.push(join_vecs(word_list[i].clone(), word_list[j].clone()));
                }
            }
        }
    }

    word_pairs
}

fn join_vecs(mut vec_a: Vec<char>, mut vec_b: Vec<char>) -> Vec<char> {
    vec_a.append(&mut vec_b);

    vec_a
}

// Debug output

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