//! This is a comment for the module

/// This is a comment for the blubb function
pub fn blubb() {
    println!("blubb")
}

/// Parse an input string to an unsigned integer
///
/// ```
/// # let input = "123";
/// let value = s07_code_samples_setup::mymod::mkuint(input);
/// # assert_eq!(value, 123);
/// ```
pub fn mkuint (input: &str ) -> u128 {
    input.parse::<u128>().unwrap()
}
    