# Count newlines really fast

Benchmark results on my machine (Core i5-4200M):

```
running 25 tests
test check ... ignored
test test_fast_newlines               ... bench:       2,175 ns/iter (+/- 620)
test test_fast_nonewlines             ... bench:       2,176 ns/iter (+/- 344)
test test_fast_random                 ... bench:      32,466 ns/iter (+/- 983)
test test_fast_somenewlines           ... bench:          16 ns/iter (+/- 2)
test test_faster_newlines             ... bench:       2,940 ns/iter (+/- 527)
test test_faster_nonewlines           ... bench:       2,938 ns/iter (+/- 956)
test test_faster_random               ... bench:      43,954 ns/iter (+/- 1,720)
test test_faster_somenewlines         ... bench:          25 ns/iter (+/- 5)
test test_fastest_newlines            ... bench:       1,286 ns/iter (+/- 167)
test test_fastest_nonewlines          ... bench:       1,284 ns/iter (+/- 383)
test test_fastest_random              ... bench:      19,039 ns/iter (+/- 5,534)
test test_fastest_somenewlines        ... bench:          14 ns/iter (+/- 3)
test test_hyperscreaming_newlines     ... bench:         716 ns/iter (+/- 90)
test test_hyperscreaming_nonewlines   ... bench:         712 ns/iter (+/- 239)
test test_hyperscreaming_random       ... bench:      10,901 ns/iter (+/- 492)
test test_hyperscreaming_somenewlines ... bench:          12 ns/iter (+/- 2)
test test_screaming_newlines          ... bench:       1,180 ns/iter (+/- 258)
test test_screaming_nonewlines        ... bench:       1,180 ns/iter (+/- 140)
test test_screaming_random            ... bench:      17,565 ns/iter (+/- 740)
test test_screaming_somenewlines      ... bench:          10 ns/iter (+/- 3)
test test_slow_newlines               ... bench:       5,698 ns/iter (+/- 814)
test test_slow_nonewlines             ... bench:       3,704 ns/iter (+/- 797)
test test_slow_random                 ... bench:      67,856 ns/iter (+/- 2,345)
test test_slow_somenewlines           ... bench:          31 ns/iter (+/- 2)
```

I should note that this machine does have POPCNT, though it does not appear to
make any difference.
