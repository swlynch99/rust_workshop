// In a `lib.rs`, `mod.rs` or `main.rs` file, the `mod` keyword defines what things are
// compiled into the current module (i.e., the current main file).
// This is different from the `use` keyword, which brings things into scope. You can
// always use a fully-qualified path at each use site, but that can be verbose. The `use`
// keyword can cut down on line noise. There's a little art to picking the right balance
// between `use` and fully or partially-qualified paths. You'll see with time what works
// best for you and your team.
mod example_arguments;
mod example_string_processor;

use example_string_processor::ExampleNumberOrString;

fn main() {
    // This is a convenient way to set up the normal Rust ecosystem logging.
    env_logger::init();

    // Parse your arguments.
    let args = example_arguments::parse_arguments();

    // Handle your argument - numbers versus strings.
    let number_or_string = ExampleNumberOrString::new(args.input);

    // Print the result.
    // This is a match statement. It's like a switch statement in other languages, but in Rust
    // matches are powerful. They destructure types (though that's not unique to match) and they
    // are "exhaustive." This means that you have to handle every possible case of the enum.
    // That might sound annoying, particularly if you add a new variant to the enum later, but
    // consider for a moment: What would happen if you didn't handle that new variant? It would
    // silently compile then blow up at runtime.
    // By using `match` on an enum, you protect $future_you from forgetting to update this code
    // when you add a new feature that this code must handle. The compiler helps you remember!
    match number_or_string {
        ExampleNumberOrString::Integer { integer } => {
            // This print expression uses the default format, which uses the std::fmt::Display
            // implementation for the data type. The default Display implementation for integers
            // is base 10, like you would probably expect.
            // The print expression can take positional arguments like printf. To use a positional
            // argument you omit a name. Remember, {name:format_customization} is the form here.
            println!("{}", integer * 2)
        }
        ExampleNumberOrString::Float { floating_point } => {
            // This print expression uses the format `.2`, which means "print two decimal places"
            println!("{:.2}", floating_point * 2.0)
        }
        // Since these cases are all just single expressions, you can skip the curly braces.
        // If you skip the curly braces though, you have to append a comma to separate the cases.
        ExampleNumberOrString::String { input } => println!("You said \"{input}\""),
    }
}
