You know how to write code, but you don't know how to write Rust.

That's okay, you'll figure it out. First of all, you need to be prepared to have the
compiler tell you repeatedly that you're doing something wrong for reasons that are
initially unclear to you. It's not punishing you! It's helping move your bugs up to
compile time rather than run-time. It's going to tell you about a lot of bugs though.
That's just what it does, and your code will tend to work better when you run it as a
result of this!

We're going to follow a few guidelines throughout this workshop, that will
help you develop good Rust code:
1. Read compiler and Clippy error messages. They tell you what you need to know to write what you are writing.
2. Do not use `unwrap()`. It hides your bugs from the compiler and causes crashes at runtime.
3. Favor correctness and clarity over line count and cleverness - but do not repeat yourself.

<details>
 <summary>How do I compile code?</summary>
Run this at the root of your project:

<code>
cargo build
</code>
</details>

<details>
 <summary>How do I run code?</summary>
Run this at the root of your project:

<code>
cargo run --bin intro
</code>
</details>

<details>
 <summary>What is clippy?</summary>
Clippy is a helpful linter that tells you about things that may work fine, but should
be done differently for various reasons. It often tells you what to do instead, and
sometimes it even tells you why! To run clippy, run this:

<code>
cargo clippy --all-targets
</code>
</details>

<details>
 <summary>How should I format my code?</summary>
Just use rust format, which will format your code in a usually-reasonable way:

<code>
cargo fmt
</code>
</details>

# Your mission, should you choose to accept it
Write a command line application that takes an argument and does the following:

* If the argument is a number, double it and print it out.
* If the argument is not a number, print `You said "<the argument>"`

**Why should I do this?**
This exercise familiarizes you with the popular argument parsing library
`clap`, and takes you through error handling for string parsing. It
introduces structs, macros, and rust's print formatting.

You should figure about 30 minutes if you're a Rust beginner, and 5-10 minutes
if you know what you're doing. If you don't know how to write code, no problem!
You might spend most of your workshop working on this, and it's still time well
spent ❤️
