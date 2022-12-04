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
    
    let solution = solve(word_lists.clone()); // Todo: Remove the clone() ?

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    
    
    // DEBUG output

    println!();
    print_word_lists_hashmap(word_lists_hashmap.clone(), letters.clone());
    println!();
    println!("first item, first list: {:?}", word_lists[0][0].clone());
    println!();

    let first_word = vec!['y', 'u', 'r', 't', 's'];
    let (result, index, word) = find_next_word(first_word.clone(), word_lists[4].clone(), 0);
    println!("test 1: {:?} {:?}", first_word, (result, index, word));

    let (result, index, word) = find_next_word(first_word.clone(), word_lists[4].clone(), 25);
    println!("test 2: {:?} {:?}", first_word, (result, index, word));

    let (result, index, word) = find_next_word(vec!['i', 'i', 'i', 'i', 'i'], word_lists[4].clone(), 25);
    println!("test 3: {:?} {:?}", vec!['i', 'i', 'i', 'i', 'i'], (result, index, word));

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

// Given a word, a word list & a starting index, iterates over single word list,
// looking for a word that can be joined to given word, starting from the given index.
// If found, then it returns: (true, index of word that was found, word found)
// Else returns:              (false, 0, empty vec)

fn solve(word_lists: Vec<Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let mut solutions: Vec<Vec<char>> = vec![];
    
    // Iterate over every word in each list in word_lists, except last 4 lists
    for i in 0..22 {
        let word_list = word_lists[i].clone();

        for j in 0..word_list.len() {
            // Do clever things that exhaust all potential searches for suitable 5-word chain (efficiently)
            // where word_list[j] would be 1st word in any such chain found.

            let mut word = word_list[j].clone(); // Not sure we need this!
            let mut chain: Vec<char> = word.clone();
            let mut chain_n: usize = 1; // length of current chain 1 - 5
            let mut list_index = i + 1; // index of list to search (start at next list)
            let mut start = 0; // index in list to start/continue searching from (start at beginning of next list)
            // let mut last_in_list = word_lists[0].len() -1;

            // Store of last start position at each chain_n. HashMap<chain_n, (list, index)>
            let mut regressions: HashMap<usize, (usize, usize)> = HashMap::from([(2, (0, 0)), (3, (0, 0)), (4, (0, 0)), (5, (0, 0))]);

            loop {
                // This needs the chain as a single word vec (Vec<chars>)
                // So how do we store the chain? What is most efficient?
                let (found_next, found_at, found_word) = find_next_word(chain.clone(), word_lists[list_index].clone(), start);      
                
                // Maybe we should deal with found_next == false first.
                // I suspect this simplifies the found_next == true case

                if found_next == false {
                    // Are there enough lists left to possibly build 5-word-chain?
                    // Yes, go to next list
                    if chain_possible_from_list(list_index + 1, chain_n) == true {
                        list_index += 1;
                        start = 0;
                        break;
                    } else { // Search for regression point to continue from
                        loop {
                            chain_n -= 1;

                            if chain_n == 1 { // We exhausted the last possible 2nd word list, so it's over for this 1st word.
                                break;
                            } else {
                                chain = remove_word(chain.clone());

                                let (regression_list, regression_index) = regressions[&chain_n];
                                // if regression point is end of list
                                if regression_index == word_lists[regression_list].len() - 1 {

                                } else { // Continue from regression point
                                    list_index = regression_index;
                                    start = regression_index + 1;
                                    break;
                                }
                            }
    
                        }
                    }
                    // No, regress to chain - 1 word (break if already at chain_n == 1, and then break again in this case)
                }

                if chain_n == 1 { break; } // We have failed to a 2nd word from last possible list for 2nd words. 

                // Found a word. Add it to chain.
                // if found_next == true {
                //     chain = add_word(chain.clone(), found_word.clone());
                //     *regressions.get_mut(&chain_n).unwrap() = (list_index, found_at); // Store the regression point
                //     chain_n += 1;

                //     // Found a 5-word chain. Add chain to solutions.
                //     if chain_n == 5 {
                //         solutions.push(chain.clone());

                //         println!("{:?}: {:?}", solutions.len(), chain.clone()); // DEBUG
                //     }
                // }

                // Find next starting point (and set the chain_n & chain contents accordingly)
                //
                // What should this return / provide (set)?
                //   List index, starting index, and chain_n of next starting point.
                //   Truncated chain, if chain_n reduced

                // If found_next == true
                //   If chain_n < 5
                //     (A) If next list is not too close to end of word_lists
                //       (B) Set first index of next list as start
                //     Else (C) step back to chain_n level last word discovery point = regression
                //       If regression is end of list do check (A)
                //         If passes. do (B)
                //         If fails do (C)
                //     Else new start = regression + 1 position
                //
                // If chain == 5 (special case, found a 5-word chain)

                if found_next == true {
                    // If chain_n < 5
                    loop {
                        if chain_possible_from_list(list_index + 1, chain_n) == true {
                            list_index += 1;
                            start = 0;
                            break;
                        } else {
                            chain_n -= 1;
                            chain = remove_word(chain.clone());

                            let (regression_list, regression_index) = regressions[&chain_n];
                            // if regression point is end of list
                            if regression_index == word_lists[regression_list].len() - 1 {

                            } else {
                                list_index = regression_index;
                                start = regression_index + 1;
                                break;
                            }
                             
                            // else continue from regression point
                        }
                    }
                }

                // Else (found_next) == false

                // If found a 2, 3, or 4-word chain: found_next == true
                // 
                // (A) Then, move on to next list and look for next word in chain...
                // Unless not enough lists left to make 5-word chain, in which case
                // Go back to previous level and continue looking from that point in that list...
                // Unless already at end of that list...
                // In which case go to next list, unless not enough lists left to make 5-word chain... which is a loop with start at (A)
                // Break if we get to chain_n == 1 at last word in the current list[i]
                //
                // So, we need a function that does this and returns either the chain-level (chain_n), list index and index in list of a valid starting point...
                // or returns something that signals search is exhausted


                // If failed to find a 2, 3, 4 or 5 word chain: found next == false
                //
                // Basically, do the same as in loop (A), above. Any differences?

                // Condition for breaking out to next starting word iteration (in j) - Need to check this!
                // if found_next == false && chain_n == 1 && start == last_in_list { break; } // Exhausted trees possible, starting from words in this list

                // find_next_word
            }
        }
    }

    solutions
}

fn find_next_word(chain: Vec<char>, list: Vec<Vec<char>>, start: usize) -> (bool, usize, Vec<char>) {
    for i in start..list.len() {
        if vecs_have_no_dups(chain.clone(), list[i].clone()) {
            return (true, i, list[i].clone());
        }
    }

    (false, 0, vec![])
}

// Add word (Vec<char>) to word-chain (Vec<char>)
fn add_word(mut chain: Vec<char>, word: Vec<char>) -> Vec<char> {
    for i in 0..5 {
        chain.push(word[i]);
    }

    chain
}

// Remove last word (Vec<char>) from word-chain (Vec<char>)
fn remove_word(mut chain: Vec<char>) -> Vec<char> {
    for _i in 0..5 {
        chain.pop();
    }

    chain
}

fn chain_possible_from_list(list_index: usize, chain_n: usize) -> bool {
    if (list_index + 5 - chain_n) > 25 { return false; }

    true
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

fn print_word_list_sizes(word_lists: Vec<Vec<Vec<char>>>) {
    for i in 0..26 {
        print!("{:?} ", word_lists[i].len());
    }

    println!()
}

// fn print_word_pairs(word_pairs: Vec<Vec<Vec<char>>>) {
//     for (_i, pair) in word_pairs.iter().enumerate() {
//         println!("{:?}", pair);
//     }
// }




// Store of unused functions

// fn concatenate_vecs(mut vec_a: Vec<char>, mut vec_b: Vec<char>) -> Vec<char> { 
//     vec_a.append(&mut vec_b);

//     vec_a.clone()
// }

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