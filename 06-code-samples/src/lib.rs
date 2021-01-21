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
//! 
//! # Code Samples
//! 
//! Code Samples are not only rendered nicely, they are also used to write 
//! doctests, which means that you can write documentation and tests at
//! the same time. 
//! 
//! As of January 2021 this only works so-so for public methods, so it's 
//! nice to have but not the best way to add testing to your library. It 
//! does not yet work with cargo and binaries.

pub mod mymod;

/// This is a comment for the main function
pub fn main() {
    println!("Hello, world!");
    mymod::blubb();
    mymod::mkuint("123");
}
