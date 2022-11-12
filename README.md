# regex-benchmarks

This is a benchmark of two regular expression quantifiers:

- The [possessive quantifier][possessive], the `+` in e.g. `.*+`
- The [non-greedy quantifier][non-greedy], the `?` in e.g. `.*?`

[possessive]: https://www.regular-expressions.info/possessive.html
[non-greedy]: https://www.regular-expressions.info/repeat.html

In particular, I'd like to know if using them makes regex matching faster or
slower.

Disclaimer: We're testing with Rust's [`regex` crate][regex-crate], which is
loosely based on the RE2 library. The conclusion likely only applies to this
crate and `#notallregexlibraries`! Different regex libraries may apply vastly
different optimizations and therefore have different costs associated.

[regex-crate]: https://docs.rs/regex/latest/regex/

## The experiment

Three regular expressions are being compared:

- `'([^']*)'`
- `'([^']*+)'`
- `'(.*?)'`

against two types of quoted strings with `...` being 500 random alphanumeric bytes:

- `'...'` which succeeds matching all three regexes
- `'...` which fails matching all three regexes

The crux of these six benchmarks is the expression `regex.captures(&input)`.

As a control experiment, three non-capturing variants are also benchmarked:

- `'[^']*'`
- `'[^']*+'`
- `'.*?'`

These are tested both as successful and failing matches. This helps assess
whether the performance difference between the three is less than the cost of
capturing the result.

The crux of the six control benchmarks is the expression `regex.is_match(&input)`.

## Questions

- The possessive quantifier is expectably faster, but how much?
- The non-greedy quantifier is expectably slower, but how much?
- Is the speed difference above or below the cost of capturing groups?

## Answers

Condensing the [`cargo bench`](#appendix-cargo-bench) results,

- The possessive quantifier is actually insignificantly slower than omitting it.
- The non-greedy quantifier is twice as slow, but still insignificantly slower.
- Capturing groups have approximately a factor 7 bigger impact on the performance
  of a regular expression than a quantifier

```
capturing regexes that succeed/normal/500 time:     [4.7930 µs 4.7942 µs 4.7956 µs]
capturing regexes that succeed/possessive/500 time: [4.8243 µs 4.8256 µs 4.8269 µs]
capturing regexes that succeed/non-greedy/500 time: [8.7937 µs 8.7961 µs 8.7986 µs]

capturing regexes that fail/normal/500 time:        [676.01 ns 676.15 ns 676.29 ns]
capturing regexes that fail/possessive/500 time:    [684.92 ns 685.06 ns 685.20 ns]
capturing regexes that fail/non-greedy/500 time:    [695.49 ns 695.65 ns 695.82 ns]

non-capturing regexes that succeed/normal/500 time:     [679.22 ns 679.41 ns 679.59 ns]
non-capturing regexes that succeed/possessive/500 time: [683.18 ns 683.34 ns 683.51 ns]
non-capturing regexes that succeed/non-greedy/500 time: [685.60 ns 685.78 ns 685.97 ns]

non-capturing regexes that fail/normal/500 time:        [703.49 ns 703.69 ns 703.90 ns]
non-capturing regexes that fail/possessive/500 time:    [692.07 ns 692.25 ns 692.44 ns]
non-capturing regexes that fail/non-greedy/500 time:    [694.48 ns 694.64 ns 694.80 ns]
```

## Conclusions

- Don't bother writing `'[^']*+'` instead of `'[^']*'`: Slightly slower and less readable.
- If you think `'.*?'` is more readable, it's probably not slower at a scale that matters.
- Remember to use [non-capturing groups][non-capture] if you don't actually need to extract the contents.

[non-capture]: https://www.regular-expressions.info/brackets.html

## Appendix: `cargo bench`

```
     Running benches/regex.rs (target/release/deps/regex-84f60dd518617320)
capturing regexes that succeed/normal/500
                        time:   [4.7930 µs 4.7942 µs 4.7956 µs]
Found 132 outliers among 10000 measurements (1.32%)
  97 (0.97%) high mild
  35 (0.35%) high severe
capturing regexes that succeed/possessive/500
                        time:   [4.8243 µs 4.8256 µs 4.8269 µs]
Found 143 outliers among 10000 measurements (1.43%)
  141 (1.41%) high mild
  2 (0.02%) high severe
capturing regexes that succeed/non-greedy/500
                        time:   [8.7937 µs 8.7961 µs 8.7986 µs]
Found 17 outliers among 10000 measurements (0.17%)
  14 (0.14%) high mild
  3 (0.03%) high severe

capturing regexes that fail/normal/500
                        time:   [676.01 ns 676.15 ns 676.29 ns]
Found 1912 outliers among 10000 measurements (19.12%)
  733 (7.33%) high mild
  1179 (11.79%) high severe
capturing regexes that fail/possessive/500
                        time:   [684.92 ns 685.06 ns 685.20 ns]
Found 1172 outliers among 10000 measurements (11.72%)
  720 (7.20%) low mild
  432 (4.32%) high mild
  20 (0.20%) high severe
capturing regexes that fail/non-greedy/500
                        time:   [695.49 ns 695.65 ns 695.82 ns]
Found 13 outliers among 10000 measurements (0.13%)
  13 (0.13%) high mild

non-capturing regexes that succeed/normal/500
                        time:   [679.22 ns 679.41 ns 679.59 ns]
Found 4488 outliers among 10000 measurements (44.88%)
  2096 (20.96%) low severe
  30 (0.30%) low mild
  422 (4.22%) high mild
  1940 (19.40%) high severe
non-capturing regexes that succeed/possessive/500
                        time:   [683.18 ns 683.34 ns 683.51 ns]
Found 2279 outliers among 10000 measurements (22.79%)
  27 (0.27%) low severe
  1 (0.01%) low mild
  142 (1.42%) high mild
  2109 (21.09%) high severe
non-capturing regexes that succeed/non-greedy/500
                        time:   [685.60 ns 685.78 ns 685.97 ns]
Found 137 outliers among 10000 measurements (1.37%)
  137 (1.37%) high mild

non-capturing regexes that fail/normal/500
                        time:   [703.49 ns 703.69 ns 703.90 ns]
Found 283 outliers among 10000 measurements (2.83%)
  279 (2.79%) high mild
  4 (0.04%) high severe
non-capturing regexes that fail/possessive/500
                        time:   [692.07 ns 692.25 ns 692.44 ns]
Found 4801 outliers among 10000 measurements (48.01%)
  2380 (23.80%) low severe
  21 (0.21%) low mild
  654 (6.54%) high mild
  1746 (17.46%) high severe
non-capturing regexes that fail/non-greedy/500
                        time:   [694.48 ns 694.64 ns 694.80 ns]
Found 2516 outliers among 10000 measurements (25.16%)
  846 (8.46%) low severe
  22 (0.22%) low mild
  334 (3.34%) high mild
  1314 (13.14%) high severe
```

## 
