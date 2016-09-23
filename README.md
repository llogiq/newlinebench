# Count newlines really fast

Benchmark results on my machine (Core i5-4200M):

```
running 16 tests
test check ... ignored
test test_fast_newlines          ... bench:       2,175 ns/iter (+/- 57)
test test_fast_nonewlines        ... bench:       2,175 ns/iter (+/- 348)
test test_fast_somenewlines      ... bench:          16 ns/iter (+/- 3)
test test_faster_newlines        ... bench:       2,944 ns/iter (+/- 416)
test test_faster_nonewlines      ... bench:       2,946 ns/iter (+/- 76)
test test_faster_somenewlines    ... bench:          25 ns/iter (+/- 6)
test test_fastest_newlines       ... bench:       1,283 ns/iter (+/- 210)
test test_fastest_nonewlines     ... bench:       1,283 ns/iter (+/- 357)
test test_fastest_somenewlines   ... bench:          14 ns/iter (+/- 3)
test test_screaming_newlines     ... bench:       1,180 ns/iter (+/- 290)
test test_screaming_nonewlines   ... bench:       1,180 ns/iter (+/- 17)
test test_screaming_somenewlines ... bench:          10 ns/iter (+/- 2)
test test_slow_newlines          ... bench:       5,695 ns/iter (+/- 1,304)
test test_slow_nonewlines        ... bench:       3,725 ns/iter (+/- 463)
test test_slow_somenewlines      ... bench:          29 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 1 ignored; 15 measured
```

I should note that this machine does have POPCNT, though it does not appear to
make any difference. Please run the benchmark on your machine and report your
finding as an [issue](https://github.com/llogiq/newlinebench/issue/new) on this
project. Thank you!
