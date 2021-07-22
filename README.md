# Dynamic programming problems and solutions

Dynamic programming \[...\] refers to simplifying a complicated problem
by breaking it down into simpler sub-problems in a recursive manner.

https://en.wikipedia.org/wiki/Dynamic_programming

## Highlighted problems and solutions

### [Stockmax](stockmax)

Your algorithms have become so good at predicting the market that you now know
what the share price of Wooden Orange Toothpicks Inc. (WOT) will be
for the next number of days.

Each day, you can either buy one share of WOT, sell any number of shares of WOT
that you own, or not make any transaction at all. What is the maximum profit
you can obtain with an optimum trading strategy?

[READ MORE](stockmax)

```bash
cargo test --release --doc -p stockmax
```

```text
running 3 tests
test src/lib.rs - stockmax (line 7) ... ok
test src/lib.rs - stockmax (line 3) ... ok
test src/lib.rs - stockmax (line 11) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```bash
cargo run --release stockmax < stockmax/test/input/input11.txt
```

```text
6
4
```

## Mo' money 🤑 🤑 🤑 mo' problems 😩 😩 😩

This section lists problems and solutions that were not
interesting enough to deserve highlighting.

Currently there are no problems and solutions in this section.
Mainly because only one problem has been solved so far.
And partly because not interesting problems are also not so
interesting to solve so I might decide not to solve them
in the first place.
