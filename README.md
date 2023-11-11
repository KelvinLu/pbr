# PBR

> Programming Bitcoin in Rust

# FAQ

## What is this?

- An implementation of various topics covered by Jimmy Song's
  [_Programming Bitcoin_](https://programmingbitcoin.com) course.
- ... in Rust!

## Why Rust?

This project is also an exercise in learning how to program using Rust.

![Ferris](ferris.jpg?raw=true)

## How do you run the exercises?

Install Rust and Cargo.

Then, `cargo run --example <target> <n>`.

> - `target`: File to run (`exercises/*.rs`), corresponding to each chapter.
> - `n`: Which exercise to run (`1`, `2`, etc.) from the chapter.
>   - _`0` runs other miscellaneous code samples included in the chapter._

... or, `script/run-all-examples`.

## Can I get that with a side of `rustdoc`?

`cargo doc --open`.

## Why `ruint`?

Because I would like 256-bit integer support when working through the exercises.

## Can I use this in production?

Would you trust code written by an apathetic crab that is drinking a tequila
sunrise whilst rolling up some indica on the beach?

Something like [`rust-bitcoin`](https://crates.io/crates/bitcoin) or
[`rust-secp256k1`](https://crates.io/crates/secp256k1), which probably has much
more
[community support/development](https://www.youtube.com/watch?v=-50NdPawLVY&t=31s),
ought to be preferred.

## Did you have to scan your eyeballs to use the `ruint` crate?

[No, thankfully not.](https://www.youtube.com/watch?v=F3pIzRprsUs)

## Is there anything else to say?

[Okay, lessss' go!](https://www.youtube.com/watch?v=AWM5ZNdWlqw)
