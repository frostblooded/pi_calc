# Pi calculation multithreading test
Benchmark calculating Pi with and without multithreading and with and without factorial caching.

## How to run

Run

```
cargo bench
```

to generate the benches. *This may take a lot of time (for me it is usually up to 10-20 minutes).*

Then you can browse the generated files in `target/criterion`.
For example, open `target/criterion/report/index.html` to view a menu for all the benchmarks.