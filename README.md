# five words

## background

How fast can I find all 5-word lists, where each list does not contain any duplicate letters (25 unique letters), given a specific word list of English words?

Based on question posed in this video: [Someone improved my code by 40,832,277,770%](Someone improved my code by 40,832,277,770%) by Stand-up Maths

Some has done it in 6.5 microseconds (vs. the OP's > 1 month!). I'll be happy with < 1 minute (using only a single thread).

Uses this word list: https://raw.githubusercontent.com/dwyl/english-words/master/words.txt

## first naive approach

Start by reading lines in using this approach: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

Store each line as a string ('word') in a vec, but filter out 'words' before adding to vec if:
- contains non-letter character
- not five characters long
- contains any duplicate characters

Total runtime: 136 ms
Resulting Vec contains 18904 words

Idea for next step:

Iterate over Vec, stopping at n-1 element.
Convert first word to vec of chars, and then compare with all other words (also a vecs of chars), checking if combined 2 words contain no duplicate chars.
- If no duplicates, then save the pair of words in a collection, which includes the index of the words: `Vec<[Vec<char>, index, Vec<char>, index], [...], ...>`
- do the same for all words, only checking by combining with words later in Vec... this should never repeat a comparison, and will result in all possible word pairs being checked and filtered.