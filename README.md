# Boggle Solver

A Solver for the word game [Boggle].

## Installation

Install [Rust], and compile this program

```console
$ git clone https://github.com/matematikaadit/boggle-solver.git
$ cd boggle-solver
$ rustc -O boggle.rs
```

Or replace the last command with
```console
$ make
```

## Explanation and Usage

Boggle is a word game played on 4x4 grid letter. The goal is to find as many
dictionary word as possible by traversing the grid. Here an example of such
word found in one of the game

> Example of the word TOURING found in a boggle game

![touring.png]

To get all the solutions to those game above, you can run the following

```console
$ ./boggle "ingc cito iaro tkru"
```

You build the string representation of the grid by joining the letters in the
same row next to each other, and then insert one space between each row. That
invocation will use `/usr/share/dict/words` dictionary file by default. If
you're on windows, you can supply your own dictionary file as the second
argument. Each word should be separated by newline in the dictionary file.

## License

MIT License

Copyright (c) 2018 Adit Cahya Ramadhan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Resource for Online Play

- [wordplays.com/boggle][wordplays-boggle]
- full rust [online multiplayer boggle][panicbit-boggle] implementation by panicbit

## Possible Improvement and Future Ideas

- Sort the solution by the word's length so that you can input the longest word
  first
- Use Trie instead of HashSet and benchmark the running time (current
  HashSet implementation is quite fast, less than 1 second for standard 4x4 grid
  and the default dictionary file)
- Write your own boggle game: TUI, GUI, Mobile app, or maybe Web-based


[Boggle]: https://en.wikipedia.org/wiki/Boggle
[Rust]: https://rustup.rs/
[touring.png]: touring.png
[wordplays-boggle]: https://www.wordplays.com/boggle
[panicbit-boggle]: https://github.com/panicbit/boggle
