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
//! 
//! ## Setting things up
//! 
//! Using special syntax in the doctest you can even write code that is not rendered to the docs.
//! 
//! There is a lot more about doctests and some more sugar in the rustdoc book.
//! 
//! (IMHO) This gets ugly fast, so use it only for small samples.
//! # All the links 
//! 
//! rustdoc makes it really easy to link somewhere. You can just use `[mymod]` to link to [mymod] for example.
//! 
//! These links have some tricks up their sleeve, take look into the [links] mod.
//! There we are using prefixes to the name of the Struct and Function to link to the correct item. 
//! More about them in the rustdoc book.

pub mod mymod;
pub mod links;

/// This is a comment for the main function
pub fn main() {
    println!("Hello, world!");
    mymod::blubb();
    mymod::mkuint("123");
}
