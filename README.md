# stopwatch-rs

You often need a stopwatch to embed performance measuremet in the production code.
This library implements a stopwatch just like the real one which equips
suspend/resume functionality, which helps to skip code that shouldn't be included
in the measurement.

The stopwatch uses [quanta](https://github.com/metrics-rs/quanta) library there it can be built for WASM target.

## Example

```rust
let mut sw = stopwatch_rs::StopWatch::start();
sleep(Duration::from_secs(1));
println!("{}", sw.split()); // split=1s, lap=1s
sw.suspend();
sleep(Duration::from_secs(2));
sw.resume();
sleep(Duration::from_secs(4));
println!("{}", sw.split()); // split=5s, lap=4s
```