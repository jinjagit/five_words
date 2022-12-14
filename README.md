# five words

## background

How fast can I find all 5-word lists, where each list does not contain any duplicate letters (25 unique letters), given a specific word list of English words?

Based on question posed in this video: [Someone improved my code by 40,832,277,770%](https://www.youtube.com/watch?v=c33AZBnRHks) by Stand-up Maths

Someone has done it in 6.5 microseconds (vs. the OP's > 1 month!). I'll be happy with < 1 minute (using only a single thread).

Uses this word list: https://raw.githubusercontent.com/dwyl/english-words/master/words.txt

The video says the answer we are looking for is there are 538 lists of 5 words that contain 25 unique letters.

I haven't looked at any solutions to this problem, because I want to see what I come up with first.

## first naive approach

Start by reading lines in using this approach: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

Store each line ('word') as a vec of chars in a vec, but filter out 'words' before adding to vec if:
- contain non-letter character
- not five characters long
- contain any duplicate characters

Total runtime: 85 ms
Resulting vec contains 13672 words

#### Idea for next step:

Iterate over Vec, stopping at n-1 element.
Compare first word with all other words, checking if combined 2 words contain no duplicate chars.
- If no duplicates, then save the pair of words in a collection (a vec of vecs of pairs of vecs of chars)
- do the same for all words, only checking by combining with words later (than the word being considered as a 'first' word of five) in Vec... this should never repeat a comparison, and will result in all possible word pairs being checked and filtered.
- use similar process to see which words can be added to the 2-word pairs list
- now we have a list of word pairs and word triplets that all contain unique letters, so now just need a smart way to check which word pairs can be added to which word triplets.

But, finding word pairs with unique letters gives:
Total runtime: 10121 ms
word list length: 13672
word pairs length: 13491017

This seems like way too many word pairs to then iterate over, looking for which word we can add from our word_list of 5-char words!

But, I realised something, while thinking about this. Each word in the 5-word list must start with a different letter.
So, at least, we could divide the word_list into 26 groups (by first letter) and only ever try concatenating words from different groups.
Can we extend this idea further to optimize further?

look at word1. Then only iterate over words from the groups (from the 26 groups) that don't start with any letter in the word = 21 groups, to get word pairs.

Then we can try combining word_pairs, which means we can rule out 10 groups (of pairs) each time = only consider 16 groups each time. Still likely to mean approx. 13m * 8m test ops! (and we would still only have 4-word combos, so more to do)

-----------------------------------------

Hmmm, maybe we should try going all the way to 5 words from each word, only moving on when we have proved we can't use that word in any 5-word list...

This means we can stop when we reach words beginning with 'w', since will have tested the words following this point in combination with those preceeding them already, and there cannot be 5 word combos starting from this point & only adding words ahead in the vec.

I might be over-thinking this. maybe it's simpler to use the optimization to slightly improve finding word_pairs, then move to finding word_triplets (only need to try words from 16 groups each time), since there are less individual words that word pairs (by many magnitudes)... maybe the list of word_triplets will be orders of magnitude smaller than the word pairs list? This step would be approx. 13m * 8k (much better).

-----------------------------------------

If we use the 26 word lists approach:

If we focus on testing each word as a starting word (from list n, where n is list_index(by alphabetical order, but probably doesn't have to be!)):
We iterate over lists, (ignoring 5 + n lists) if we fail to build a suitable pair (if exhaust list 26-5-n-3), we move on to next word as starting word...
if we build a word pair, store the index of the word we have paired with, then move on to iterating over lists again from next list (now ignoring 10 lists)...
... if not, we return to next 2nd word (or if exhaust list 26-10-i-2)
if we build a triplet, store index, iterate again from next list (ignoring 15 lists)... if not, we return to next 2nd word (or if exhaust penulitmate list 26-15-n-1)
if we build a quartet, store index, iterate again from next list (ignoring 20 lists)... if not, we return to next 3rd word (or if exhaust last list 26-20-n, which is just the last list 'z'... just go to the end if we have quartet!)
if we build a quintet, store quintet, iterate again from next word (ignoring 20 lists) (limit = finish last list)

Need to keep track of:
n-words-chained (1-5) n_words
list of last successfully added word or starting word (0-25) list_index
index of last successfully added word or starting word (0-some usize) word_index

start at n_words, list_index, word_index (1, 0, 0)

iterate over words, starting in next list 

fn search_lists(start_list, end_list, word) {

  if find_word to add {
    return new longer word + n_words(now += 1), list_index, word_index
  }

  word(original) + n_words, list_index, word_index
}

Then, we can tell if we got anything by n_words (if no change, then found nothing)
Make decision on next search based on what we get back
? how to know when to finish ? (when we are ready to process first word in 23nd list, at n_words == 1, as cannot make 5 word chain from there!)


## super-naive approaches to avoid
1. Calculate all possible permutations of 25 letters possible from all 26 letters = 403291461126605635584000000, or 4.03 * 10^26
2. Step through each permutation, checking if each 5-letter word it contains is in the provided word list. Reject permutation as soon as one of 5 words it contains not found, or permutation contains duplicate words, or accept if all 5 different words found.

1. Calculate all possible permutations of 5-word lists possible from our filtered list of suitable words (13672) = 4738731385347960, or 4.74 * 10^15
2. Check each permutation's 5 words combined for duplicate characters, if none found add to set of solutions.

These are both super-naive ways to brute-force solve the problem, and are likely to take a loooong time to run!

Resource: https://www.calculatorsoup.com/calculators/discretemathematics/permutations.php