You know how to write code, but you don't know how to write Rust.

That's okay, you'll figure it out. First of all, you need to be
emotionally prepared to have the compiler tell you repeatedly that
you're doing something wrong. It's not punishing you! It's helping
move your bugs up to compile time rather than run-time. It's going
to tell you about a lot of bugs though, okay? That's just what it
does, and your code will tend to work better when you run it as a
result!

We're going to follow a few guidelines throughout this workshop, that will
help you be a good Rust developer:
1. Read compiler and Clippy error messages. They tell you what you need to know to write what you are writing.
2. Do not use `unwrap()`. It hides your bugs from the compiler and causes crashes at runtime.
3. Favor correctness and clarity over line count and cleverness - but do not repeat yourself.

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
