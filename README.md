# Wordle-Solver

Solves all words in the wordle dictionary in under 6 moves

## How it works
1. Keep track of what letters are possible for each position of the word.
   1. When we guess, take the feedback and adjust the possible letters in each position accordingly
   2. If we get a green letter, then we know the only possible letter in that position is that letter
   3. If we get a yellow letter, then we know the yellow letter cannot appear in that position
   4. If we get a grey letter, then we know it doesn't appear in any positions
      1. Unless the grey letter showed up twice in our guess. If the other times weren't grey, then we can only be sure that the letter doesn't appear in the grey position, not necessarily all positions.
2. First we check if we've solved the word. Remove all words that don't follow the possible letters in each position as per above
   1. Also remove any words that don't contain all yellow and green verified letters
   2. If there is only one word remaining, that's the answer. If there are none, it's impossible and someone (probably me) messed up
3. If we haven't solved it, we need to make a guess that maximizes the possibility of eliminating other words.
   1. So for each position in the words, we record the frequencies of each letter per position
   2. Similarly, we record for each letter, how many words it shows up
   3. Now we calculate a score for each word that we are allowed to use for guessing
      1. Sum up the letter frequency per position for every character in the guess word (use 3i)
      2. Also add how many other words share any letter with the guess word (use 3ii)
         1. multiply this by 2 because it helps to put more weight
   4. Take the word with the highest score

## Stats
| Number of tries | Number of words |
|-----------------|-----------------|
| 2               | 22              |
| 3               | 764             |
| 4               | 1280            |
| 5               | 241             |
| 6               | 8               |
| **Average**     | **3.65**        |

The 8 words that needed six tries are: `hatch`, `chill`, `catch`, `chili`, `waver`, `wafer`, `fight`, and `tight`.

This solver is far from perfect, but it works really well.
