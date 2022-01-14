import sys
import json
import random
from collections import defaultdict, Counter

ALPHABET = 'abcdefghijklmnopqrstuvwxyz'

def inv_dist(n, g):
    return int(g/2 - abs(g/2 - n))

class Solution:
    def __init__(self, word_len, target_words, guess_words, debug=False):
        self.word_len = word_len
        self.target_words = target_words
        self.guess_words = guess_words
        self.letters = [set(ALPHABET) for _ in range(word_len)]
        self.tried_letters = set()
        self.debug = debug
        self.first_time = True
        self.known_good = set()
        self.known_bad = set()

    def print(self, *args, **kwargs):
        if self.debug:
            print(*args, **kwargs)

    def guess(self):
        choices = [w for w in self.target_words if all(l in p for l,p in zip(w, self.letters))]
        choices = [w for w in choices if all(l in w for l in self.known_good)]

        remaining_letters = []
        for i in range(self.word_len):
            remaining_letters.append(set(w[i] for w in choices))

        self.letters = remaining_letters

        if len(choices) < 20:
            self.print(choices)

        if len(choices) == 1:
            return choices[0]
        elif len(choices) == 0:
            raise Exception("Impossible")

        letter_freqs = []

        for i in range(self.word_len):
            letter_freqs.append({l: inv_dist(v, len(choices)) for l,v in Counter(w[i] for w in choices).items()})

        word_freq = {l: inv_dist(v, len(choices)) for l,v in Counter(l for w in choices for l in set(w)).items()}

        self.print(letter_freqs)
        self.print(word_freq)

        scores = {}

        for w in self.guess_words:
            scores[w] = 0
            for i,l in enumerate(w):
                scores[w] += (letter_freqs[i].get(l,0))
            for l in set(w):
                scores[w] += (word_freq.get(l,0)) * 2

        sorted_scores = sorted(scores.items(), key=lambda x:-x[1])
        self.print(sorted_scores[:10])
        return sorted_scores[0][0]

    def update(self, word, result):
        pairs = list(zip(result.upper(), word))
        self.print(self.letters)

        for i,(r,l) in enumerate(pairs):
            if r == 'G': # grey
                if ('Y', l) in pairs or ('E', l) in pairs:
                    self.letters[i].discard(l)
                else:
                    for lett in self.letters:
                        lett.discard(l)
                    self.known_bad.add(l)

            elif r == 'Y': # yellow
                self.letters[i].discard(l)
                self.known_good.add(l)

            elif r == 'E': # green
                self.letters[i] = set()
                self.letters[i].add(l)
                self.known_good.add(l)

            self.tried_letters.add(l)

        self.print(self.letters)


def main():
    WORD_LEN = 5
    GREY = '\x1b[0;37;40m'
    GREEN = '\x1b[1;32;40m'
    YELLOW = '\x1b[1;33;40m'
    CLEAR = '\x1b[0m'

    targets = open('wordle_targets.txt', 'r').read().strip().split('\n')
    targets = [w for w in targets if len(w) == WORD_LEN]

    dictionary = open('wordle_dictionary.txt', 'r').read().strip().split('\n')
    dictionary = [w for w in dictionary if len(w) == WORD_LEN]

    interactive = False
    debug = False

    if len(sys.argv) == 2:
        if sys.argv[1] == '-i':
            interactive = True
            debug = True
        elif len(sys.argv[1]) == WORD_LEN:
            word = sys.argv[1]
            debug = True
        else:
            raise Exception('Bad args')
    else:
        word = random.choice(targets)
        print(f'Mystery word: {word}')

    s = Solution(WORD_LEN, targets, dictionary, debug)
    tries = 0

    previous = []

    if interactive:
        while True:
            guess = s.guess()
            print(guess)
            s.update(guess, input().strip())

    else:
        while True:
            guess = s.guess()
            count = Counter(word)

            result = [None] * WORD_LEN

            for i,(gl,wl) in enumerate(zip(guess, word)):
                if gl == wl:
                    result[i] = 'E'
                    count[wl] -= 1

            for i,(gl,wl) in enumerate(zip(guess, word)):
                if result[i] != None:
                    pass

                elif count.get(gl,0) > 0:
                    result[i] = 'Y'
                    count[gl] -= 1

                else:
                    result[i] = 'G'


            for gl,r in zip(guess, result):
                if r == 'E':
                    print(GREEN, end='')
                elif r == 'Y':
                    print(YELLOW, end='')
                elif r == 'G':
                    print(GREY, end='')
                print(gl, end='')
            print(CLEAR)

            s.update(guess, ''.join(result))
            tries += 1

            if guess in previous:
                raise Exception("Failed")
            
            previous.append(guess)

            if guess == word:
                print(f'You did it in {tries} tries!')
                break

            elif tries > 6:
                raise Exception('Uh oh...')
                break

if __name__ == '__main__':
    main()
