# five words

## background

How fast can I find all 5-word lists, where each list does not contain any duplicate letters (25 unique letters), given a specific word list of English words?

Based on question posed in this video: [Someone improved my code by 40,832,277,770%](https://www.youtube.com/watch?v=c33AZBnRHks) by Stand-up Maths

Some has done it in 6.5 microseconds (vs. the OP's > 1 month!). I'll be happy with < 1 minute (using only a single thread).

Uses this word list: https://raw.githubusercontent.com/dwyl/english-words/master/words.txt

## first naive approach

Start by reading lines in using this approach: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

Store each line ('word') as a vec of chars in a vec, but filter out 'words' before adding to vec if:
- contain non-letter character
- not five characters long
- contain any duplicate characters

Total runtime: 80 ms
Resulting Vec contains 18904 words

Idea for next step:

Iterate over Vec, stopping at n-1 element.
Compare first word with all other words (also a vecs of chars), checking if combined 2 words contain no duplicate chars.
- If no duplicates, then save the pair of words in a collection, which includes the index of the words: `Vec<[Vec<char>, Vec<char>, [index, index]], [...], ...>`
- do the same for all words, only checking by combining with words later in Vec... this should never repeat a comparison, and will result in all possible word pairs being checked and filtered.
- use similar process to see which words can be added to the 2-word pairs list
- now we have a list of word pairs and word triplets that all contain unique letters, so now just need a smart way to check which word pairs can be added to which word triplets.

Maybe it makes more sense to convert each word to a vec of chars when first storing them in a vec: `Vec<Vec<char>>`
