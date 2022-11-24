use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;
use std::collections::HashMap;

fn main() {
    println!("calculating...");
    println!();
    
    let start = SystemTime::now();

    // Read the word_list file and add only suitable words to word_list Vec.
    // Words must be 5 letters, with no other character types, and no duplicate characters.
    // 80 ms

    let mut word_list: Vec<Vec<char>> = vec![];

    let letters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut word_lists_hashmap: HashMap<char, Vec<Vec<char>>> = HashMap::new();

    if let Ok(lines) = read_lines("./words.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let word = ip.to_lowercase();

                // Check word is 5 characters, and all characters are letters
                if word.chars().count() == 5 && word.chars().all(|c| matches!(c, 'a'..='z')) {
                    let char_vec: Vec<char> = word.chars().collect();
                    // Check word does not contain duplicate characters
                    if vec_has_no_dups(char_vec.clone()) {
                        word_list.push(char_vec.clone()); // TODO remove this line & maybe the clone() in line below
                        word_lists_hashmap.entry(char_vec[0]).or_insert(Vec::new()).push(char_vec.clone());
                    }
                }
            }
        }
    }

    let unordered_word_lists = word_lists_into_vec(word_lists_hashmap.clone());
    let word_lists = order_word_lists_ascending_size(unordered_word_lists.clone());

    // Find pairs of words with no shared characters
    // 10 s

    // let word_pairs: Vec<Vec<Vec<char>>> = find_word_pairs(word_list.clone());
    // let word_pairs: Vec<Vec<Vec<char>>> = find_word_pairs_2(word_lists_hashmap.clone(), letters.clone());

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    // print_word_list(word_list.clone());
    // println!();
    // print_word_pairs(word_pairs.clone());
    // println!("{:?}", word_pairs[13].clone());

    println!();
    print_word_lists_hashmap(word_lists_hashmap.clone(), letters.clone());
    println!();
    println!("first item, first list: {:?}", word_lists[0][0].clone());

    let first_word = vec!['y', 'u', 'r', 't', 's'];
    let (result, index, word) = find_next_word(first_word.clone(), word_lists[0].clone(), 0);
    println!();
    println!("test 1: {:?} {:?}", first_word, (result, index, word));

    let (result, index, word) = find_next_word(first_word.clone(), word_lists[0].clone(), 25);
    println!();
    println!("test 2: {:?} {:?}", first_word, (result, index, word));

    println!();
    print_word_list_sizes(word_lists.clone());

    println!();
    println!("Total runtime: {} ms", duration.as_millis());
    println!("word list length: {:?}", word_list.len());
    //println!("word pairs length: {:?}", word_pairs.len());
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

fn word_lists_into_vec(hashmap: HashMap<char, Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut word_lists = vec![];

    for (_key, value) in hashmap {
        word_lists.push(value);
    }

    word_lists
}

fn order_word_lists_ascending_size(mut word_lists: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut ordered_lists = vec![];

    while word_lists.len() > 0 {
        let mut smallest_index: usize = 999999;
        let mut smallest_list: usize = 999999;

        for i in 0..word_lists.len() {
            if word_lists[i].len() < smallest_list {
                smallest_list = word_lists[i].len();
                smallest_index = i;
            }
        }

        ordered_lists.push(word_lists[smallest_index].clone());
        word_lists.remove(smallest_index);
    }

    ordered_lists
}

// fn that, given a word, a word list & a starting index, iterates over single word list,
// looking for a word that joined to given word, starting from the given index.
// If found, then it returns: (true, index of word that was found, word found)
// Else returns:              (false, 0, empty vec)

fn find_next_word(word: Vec<char>, list: Vec<Vec<char>>, start: usize) -> (bool, usize, Vec<char>) {
    for i in start..list.len() {
        if vecs_have_no_dups(word.clone(), list[i].clone()) {
            return (true, i, list[i].clone());
        }
    }

    (false, 0, vec![])
}

fn print_word_list_sizes(word_lists: Vec<Vec<Vec<char>>>) {
    for i in 0..26 {
        print!("{:?} ", word_lists[i].len());
    }

    println!()
}


// Debug output

fn print_word_list(word_list: Vec<Vec<char>>) {
    for (_i, char_vec) in word_list.iter().enumerate() {
        println!("{:?}", char_vec);
    }
}

fn print_word_lists_hashmap(word_lists_hashmap: HashMap<char, Vec<Vec<char>>>, letters: Vec<char>) {
    for i in 0..25 {
        println!("{:?}:", letters[i]);

        let word_list = word_lists_hashmap[&letters[i]].clone();
        print_word_list(word_list);
    }
}

// fn print_word_pairs(word_pairs: Vec<Vec<Vec<char>>>) {
//     for (_i, pair) in word_pairs.iter().enumerate() {
//         println!("{:?}", pair);
//     }
// }

// Store of unused functions

// fn find_word_pairs(word_list: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
//     let mut word_pairs: Vec<Vec<Vec<char>>> = vec![];
//     let word_list_len = word_list.len();

//     for i in 0..word_list_len {
//         if i != word_list_len - 1 {
//             for j in (i + 1)..word_list_len {
//                 if vecs_have_no_dups(word_list[i].clone(), word_list[j].clone()) {
//                     let word_pair: Vec<Vec<char>> = vec![word_list[i].clone(), word_list[j].clone()];

//                     word_pairs.push(word_pair);
//                 }
//             }
//         }
//     }

//     word_pairs
// }

// fn find_word_pairs_2(word_lists_hashmap: HashMap<char, Vec<Vec<char>>>, letters: Vec<char>) -> Vec<Vec<Vec<char>>> {
//     let mut word_pairs: Vec<Vec<Vec<char>>> = vec![]; // TODO: Update to 26 word lists

//     for i in 0..26 { // every list
//         let word_list_1 = word_lists_hashmap[&letters[i]].clone();

//         for j in 0..word_list_1.len() { // every word
//             let word_a = word_list_1[j].clone();

//             for k in (i + 1)..26 {
//                 if !word_a.contains(&letters[k]) { // every list beyond 1st list and excluding for letters of word1
//                     let word_list_2 = word_lists_hashmap[&letters[k]].clone();

//                     for l in 0..word_list_2.len() { // every word
//                         if vecs_have_no_dups(word_list_1[j].clone(), word_list_2[l].clone()) {
//                             let word_pair: Vec<Vec<char>> = vec![word_list_1[j].clone(), word_list_2[l].clone()];
        
//                             word_pairs.push(word_pair);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     word_pairs
// }