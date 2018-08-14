use std::cmp;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque as Queue;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();

    let puzzle = if let Some(s) = args.next() {
        Puzzle::from_str(&s)
    } else {
        eprintln!(r#"USAGE: {program} <PUZZLE> [DICT_FILE]

EXAMPLE
    {program} "adut qsoa iism irta"
    {program} "adut qsoa iism irta" /usr/share/dict/words

PUZZLE
    String representation of the puzzle grid, lowercase only, each row separated
    by space. Example: "adut qsoa iism irta".

DICT_FILE
    Path to the dictionary file. Each dictionary word in the file should be
    separated by newline. Default to: "/usr/share/dict/words"
"#, program=program);
        process::exit(1);
    };

    let dict = args.next().unwrap_or("/usr/share/dict/words".to_string());
    let dict = DictBuilder::open(dict).filter(&puzzle);

    let answer = puzzle.search(&dict);
    for (i, word) in answer.iter().enumerate() {
        println!("{:>4}: {}", i, word);
    }
}

//==========================================================
// Implementation Details

/// Puzzle contains our boggle puzzle
struct Puzzle {
    /// The boggle puzzle's grid map
    /// Only lowercase char
    map: Vec<Vec<char>>,
    /// Characters contained in the puzzle grid
    chars: HashSet<char>,
    /// The puzzle's row size
    row_size: usize,
    /// The puzzle's col size
    col_size: usize,
}

impl Puzzle {
    /// Create the Puzzle from an &str
    fn from_str(s: &str) -> Puzzle {
        let mut map = Vec::new();
        let mut chars = HashSet::new();
        let mut num_rows = 0;
        let mut num_cols = None;
        for word in s.split(' ') {
            // check current word's len and compare to previous one if exist
            // all should have the same length
            let len = word.len();
            if let Some(l) = num_cols {
                assert!(l == len);
            } else {
                num_cols = Some(len);
            }

            let mut row = Vec::new();
            for c in word.chars() {
                row.push(c);
                chars.insert(c);
            }
            map.push(row);
            num_rows += 1;
        }

        Puzzle {
            map,
            chars,
            row_size: num_rows,
            col_size: num_cols.unwrap(),
        }
    }

    /// Check if a character is contained in the puzzle
    fn contains(&self, c: char) -> bool {
        self.chars.contains(&c)
    }

    /// Search all possible solutions
    fn search(&self, dict: &Dict) -> BTreeSet<String> {
        let mut answer = BTreeSet::new();
        let mut queue = Queue::new();
        // insert all cells
        for row in 0..self.row_size {
            for col in 0..self.col_size {
                let word = self.map[row][col].to_string();
                // skip if we found no word with that prefix in the dictionary
                if !dict.is_prefix(&word) {
                    continue;
                }
                let mut visited = HashSet::new();
                visited.insert((row, col));
                let item = QueueItem { row, col, word, visited };
                queue.push_back(item);
            }
        }

        // flood fill
        while !queue.is_empty() {
            let v = queue.pop_back().unwrap();
            if dict.contains(&v.word) {
                answer.insert(v.word.clone());
            }

            // skip further path if the word is not a prefix in the dictionary
            if !dict.is_prefix(&v.word) {
                continue;
            }

            // queue all neighbour
            let lo_row = v.row.saturating_sub(1);
            let lo_col = v.col.saturating_sub(1);
            let hi_row = cmp::min(v.row + 2, self.row_size); // past one
            let hi_col = cmp::min(v.col + 2, self.col_size); // past one

            for row in lo_row..hi_row {
                for col in lo_col..hi_col {
                    // skip if it's been visited
                    if v.visited.contains(&(row, col)) { continue }
                    let mut current = v.clone();
                    current.row = row;
                    current.col = col;
                    current.word.push(self.map[row][col]);
                    current.visited.insert((row, col));
                    queue.push_back(current);
                }
            }
        }

        answer
    }
}

#[derive(Clone)]
struct QueueItem {
    row: usize,
    col: usize,
    word: String,
    visited: HashSet<(usize, usize)>,
}

/// Builder for our Dictionary
struct DictBuilder {
    buff: BufReader<File>,
}

/// Our dictionary, just a set of string, and it's prefix
struct Dict {
    words: HashSet<String>,
    prefix: HashSet<String>,
}

impl Dict {
    /// Create new empty Dict
    fn new() -> Dict {
        Dict {
            words: HashSet::new(),
            prefix: HashSet::new(),
        }
    }

    /// Insert a word and all of it's prefix
    fn insert(&mut self, word: String) {
        let mut pop_word = word.clone();
        self.words.insert(word);
        while !self.prefix.contains(&pop_word) {
            self.prefix.insert(pop_word.clone());
            pop_word.pop();
            if pop_word.is_empty() {
                break;
            }
        }
    }

    /// Check if a word contained in the dictionary
    fn contains(&self, word: &String) -> bool {
        self.words.contains(word)
    }

    /// Check if a prefix contained in the dictionary
    fn is_prefix(&self, prefix: &String) -> bool {
        self.prefix.contains(prefix)
    }
}

impl DictBuilder {
    /// Given a path, construct the builder
    fn open<P: AsRef<Path>>(path: P) -> DictBuilder {
        let file = File::open(path).unwrap();
        let buff = BufReader::new(file);
        DictBuilder { buff }
    }

    /// Filter the dictionary to only contains valid words
    fn filter(self, puzzle: &Puzzle) -> Dict {
        let mut filtered = Dict::new();
        for word in self.buff.lines() {
            let mut word = word.unwrap();
            word.make_ascii_lowercase(); // only use lowercase char
            // minimum of 3 letter word
            // all letter should be contained in the puzzle
            if word.len() < 3 || word.chars().any(|c| !puzzle.contains(c)) {
                continue;
            }
            filtered.insert(word);
        }
        filtered
    }
}
