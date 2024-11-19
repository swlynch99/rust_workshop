//! **Details** about structs, traits, and dynamic dispatch in Rust.
//!
//! Structs in Rust are similar to classes in other languages.
//! The big difference here from java, c#, and the like is that they are truly static
//! and do not have any dynamic behavior. Dynamism has to be implemented when needed,
//! and there are a few ways to do it depending on your needs. The most common way is
//! through Traits, which are similar to interfaces in other languages.
//!
//! Traits are not necessarily dynamic though; they are more of a way to describe a
//! contract for the _compiler_ than a way to describe a contract for the _runtime_.
//! They can be used dynamically, through an incantation like `Box<dyn MyTrait>`, but
//! this is slower, more memory intensive, less discoverable, and less visible to your
//! debugger than simply using `MyTrait` directly.
//!
//! Enums can be used instead of `Box<dyn MyTrait>` directly. The drawback is that you
//! have to write your vtable, in essence. There are libraries that can help with this,
//! but the line count is still higher than using `Box<dyn MyTrait>` directly.
//! The upside is that your debugger works great, your code is clear and discoverable,
//! and there is typically a small performance gain over the `Box<dyn MyTrait>` approach.
//!
//! * Structs describe data and functions on that data.
//! * Traits describe functions implemented on some data to the compiler.
//! * Enums, when used for dynamic dispatch, describe a set of data kinds which can implement traits.
//! * `Box<dyn Trait>` is an opaque pointer to a "trait object" which uses a vtable to dispatch to functions.
//!
//! Avoid using `Box<dyn Trait>`, per `guideline 3` in the root readme.

use clap::Parser;

// -- separator -- //

/// A basic example of clap argument processing.
#[derive(Debug, Parser)]
pub struct Arguments {
    /// The input string. If it's a number, it will be multiplied by 2. If it's a string, it will be repeated to you.
    #[arg(required = true)]
    pub input: String,
}

/// Parse example arguments from the command line
pub fn parse_arguments() -> Arguments {
    // This function comes from the clap library and it handles printing help & exiting
    // when the args are incorrect.

    // In rust, the last expression in a block is the block's value. This means you often
    // do not write `return` in rust.
    Arguments::parse()
}
