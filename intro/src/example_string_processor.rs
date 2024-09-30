/// A parsed value that is a number or a string.
pub enum ExampleNumberOrString {
    /// The input string was an integer.
    Integer { integer: i128 },
    /// The input string was a floating point number.
    Float { floating_point: f64 },
    /// The input string was a string.
    String { input: String },
}

impl ExampleNumberOrString {
    pub fn new(input: String) -> Self {
        match input.parse() {
            Ok(integer) => ExampleNumberOrString::Integer { integer },
            Err(parse_int_error) => {
                // In rust, the format strings have some special syntax.
                // {} is how you say "print this variable"
                // : is the separator between the variable name on the left and the formatting options on the right
                // ? is the formatter that asks for the std::fmt::Debug formatter to be used to print the variable.
                //
                // The Debug formatter is usually implemented for types in Rust, and it's usually pretty informative.
                // It's a good choice for debugging output, but it can be too noisy for output intended for users.
                log::debug!("Input was not parsed as an integer: {parse_int_error:?}");

                match input.parse() {
                    Ok(number) => ExampleNumberOrString::Float {
                        floating_point: number,
                    },
                    Err(parse_float_error) => {
                        log::debug!("Input was not parsed as an integer: {parse_float_error:?}");
                        ExampleNumberOrString::String { input }
                    }
                }
            }
        }
    }
}
