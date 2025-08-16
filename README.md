# All-of-It

So you want ALL the memory? Welcome to **All-of-It**, the Rust crate that asks: "How much memory can I have?" and then politely attempts to take it all. If you ever wanted to see your system sweat, this is the library for you.

## What does it do?

`AllOfIt` is a struct that tries to allocate a vector with the maximum size your machine's free memory allows. Yes, really. It uses `sysinfo` to check how much RAM is just sitting there, doing nothing, and then says "I'll have all of it, please."

## Usage

```rust
use all_of_it::AllOfIt;

// Try to claim all the free memory. Good luck!
let all = AllOfIt::new().expect("Your system is stingy with memory");
println!("I now own {} bytes!", all.capacity());
```

## Why would I use this?

- To test your system's limits
- To impress your friends with your RAM consumption
- To make your computer question its life choices
- For science (or chaos)

## Disclaimer

This crate will not actually crash your computer (probably), but it will try its best to make your OS sweat. Use responsibly. If your system starts making weird noises, don't blame us.

## License

MIT, because why not? Take all of it.
