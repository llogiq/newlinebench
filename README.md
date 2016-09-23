# Count newlines really fast

Benchmark results on my machine:

```
test test_fast_newlines        ... bench:       2,242 ns/iter (+/- 677)
test test_fast_nonewlines      ... bench:       2,176 ns/iter (+/- 60)
test test_fast_somenewlines    ... bench:          17 ns/iter (+/- 4)
test test_faster_newlines      ... bench:       2,946 ns/iter (+/- 433)
test test_faster_nonewlines    ... bench:       2,951 ns/iter (+/- 170)
test test_faster_somenewlines  ... bench:          26 ns/iter (+/- 5)
test test_fastest_newlines     ... bench:       1,285 ns/iter (+/- 353)
test test_fastest_nonewlines   ... bench:       1,284 ns/iter (+/- 327)
test test_fastest_somenewlines ... bench:          15 ns/iter (+/- 2)
test test_slow_newlines        ... bench:       5,722 ns/iter (+/- 1,416)
test test_slow_nonewlines      ... bench:       5,029 ns/iter (+/- 205)
test test_slow_somenewlines    ... bench:          32 ns/iter (+/- 3)
```
