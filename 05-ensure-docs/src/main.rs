//! This is a comment for the whole crate.
//! 
//! # I can write markdown
//! 
//! This makes it *easy* to **highlight** details.
//! 
//! For those who want to know: It's commonmark.
//! 
//! # Ensure comments
//! 
//! You can mark missing documentation as a **compiler error**.
//! 
//! ```#![deny(missing_docs)]```
//! 
//! Or, if you don't wanna be that harsh, you can make it a warning.
//! 
//! ```#![warn(missing_docs)]```
//!
//! For libraries there is an interesting option to control code samples
//! in the same way.
//! 
//! ```#![deny(missing_doc_code_examples)]```

mod mymod;

/// This is a comment for the main function
fn main() {
    println!("Hello, world!");
    mymod::blubb();
}
