use std::collections::{HashMap, HashSet};
use rand::thread_rng;
use rand::seq::SliceRandom;

struct WordlSolver {
    word_len: usize,
    target_words: Vec<String>,
    guess_words: Vec<String>,
    letters: Vec<HashSet<char>>,
    known_good: HashSet<char>,
}

impl WordlSolver {
    pub fn new(word_len: usize, targets: Vec<String>, guesses: Vec<String>) -> WordlSolver {
        let mut all_letters = HashSet::with_capacity(26);
        for c in 'a'..='z' {
            all_letters.insert(c);
        }

        WordlSolver {
            word_len,
            target_words: targets,
            guess_words: guesses,
            letters: vec![all_letters; word_len],
            known_good: HashSet::with_capacity(word_len),
        }
    }

    pub fn guess(&self) -> String {
        let choices = self.target_words
            .iter()
            .filter(|w| {
                // Only keep words that are possible to create based on the eliminated chars
                w.chars().zip(self.letters.iter()).all(|(l, ls)| {
                    ls.contains(&l)
                })
            })
            .filter(|w| {
                // Only keep words that have all known good letters
                self.known_good.iter().all(|l| w.contains(*l))
            })
            .collect::<Vec<&String>>();

        // Solved
        if choices.len() == 1 {
            return choices[0].to_string();
        }

        if choices.is_empty() {
            unreachable!("Word is impossible to guess");
        }

        let mut letter_freqs = Vec::new();
        for i in 0..self.word_len {
            let mut h = counter(choices.iter().map(|w| w.chars().nth(i).unwrap()));
            for (_, v) in h.iter_mut() {
                *v = inv_dist(*v, choices.len() as u32)
            }
            letter_freqs.push(h);
        }

        let mut word_freq = counter(choices.iter().flat_map(|w| w.chars().collect::<HashSet<char>>().into_iter()));
        for (_, v) in word_freq.iter_mut() {
            *v = inv_dist(*v, choices.len() as u32)
        }

        self.target_words.iter().max_by_key(|w| {
            let mut score = 0;
            for (i, l) in w.chars().enumerate() {
                score += letter_freqs[i].get(&l).unwrap_or(&0);
            }

            let mut h = HashSet::with_capacity(5);
            for l in w.chars() {
                if h.contains(&l) {
                    continue;
                }
                h.insert(l);
                score += word_freq.get(&l).unwrap_or(&0);
            }
            score
        }).unwrap().to_string()
    }

    pub fn update(&mut self, word: &str, result: Vec<CharResult>) {
        let pairs = word.chars().zip(result.into_iter()).collect::<Vec<(char, CharResult)>>();

        for (i, (l, r)) in pairs.iter().enumerate() {
            match r {
                CharResult::Grey => {
                    if pairs.contains(&(*l, CharResult::Yellow)) || pairs.contains(&(*l, CharResult::Green)) {
                        self.letters[i].remove(l);
                    } else {
                        for letters in self.letters.iter_mut() {
                            letters.remove(l);
                        }
                    }
                }

                CharResult::Yellow => {
                    self.letters[i].remove(l);
                    self.known_good.insert(*l);
                }

                CharResult::Green => {
                    self.letters[i].clear();
                    self.letters[i].insert(*l);
                    self.known_good.insert(*l);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        let mut all_letters = HashSet::with_capacity(26);
        for c in 'a'..='z' {
            all_letters.insert(c);
        }

        self.letters = vec![all_letters; self.word_len];
        self.known_good.clear();
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CharResult {
    Grey,
    Yellow,
    Green,
}

fn counter<T: std::cmp::Eq + std::hash::Hash>(it: impl Iterator<Item=T>) -> HashMap<T, u32> {
    let mut h = HashMap::new();
    for e in it {
        *h.entry(e).or_insert(0) += 1;
    }
    h
}

fn inv_dist(n: u32, goal: u32) -> u32 {
    let res = (goal as i32 - (goal as i32 - 2 * n as i32).abs()) as u32 / 2;
    res
}

fn score(guess: &str, word: &str) -> Vec<CharResult> {
    let mut count = counter(word.chars());
    let mut result = vec![None; word.len()];

    for (i, (gl, wl)) in guess.chars().zip(word.chars()).enumerate() {
        if gl == wl {
            result[i] = Some(CharResult::Green);
            *count.get_mut(&wl).unwrap() -= 1;
        }
    }

    for (i, (gl, wl)) in guess.chars().zip(word.chars()).enumerate() {
        if result[i].is_some() {
            continue;

        } else if *count.get(&gl).unwrap_or(&0) > 0 {
            result[i] = Some(CharResult::Yellow);
            *count.get_mut(&gl).unwrap() -= 1;
        } else {
            result[i] = Some(CharResult::Grey);
        }
    }

    result.iter().map(|r| r.unwrap()).collect()
}

fn main() {
    const WORD_LEN: usize = 5;
    let wordle_targets = include_str!("../wordle_targets.txt")
        .lines()
        .filter(|l| l.len() == WORD_LEN)
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let wordle_dictionary = include_str!("../wordle_dictionary.txt")
        .lines()
        .filter(|l| l.len() == WORD_LEN)
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut solver = WordlSolver::new(WORD_LEN, wordle_targets.clone(), wordle_dictionary);

    let mut rng = thread_rng();
    // let word = wordle_targets.choose(&mut rng).unwrap();


    let mut solves = HashMap::new();

    for mystery_word in &wordle_targets {
        solver.reset();
        // println!("Mystery word: {mystery_word}");

        let mut tries = 0;
        while tries < 10 {
            tries += 1;
            let guess = solver.guess();
            // println!("You guessed: {guess}");
            solver.update(&guess, score(&guess, mystery_word));
            if guess == *mystery_word {
                println!("{mystery_word}, {tries}");
                // println!("You did it in {tries} tries!");
                break;
            }
        }

        *solves.entry(tries).or_insert(0) += 1;

        if tries > 9 {
            panic!("{mystery_word}");
        }
    }

    for i in 1..10 {
        let n = solves.get(&i).unwrap_or(&0);
        println!("{i}: {n}");
    }

}
