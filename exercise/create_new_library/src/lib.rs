//! This is package to provide utility functions for reading from command line
//! Example:
//! ```
//! use create_new_library::read_stdin;
//! let input = read_stdin();
//! ```
//! panics:
//! the 'read_stdin' function will panic if it fails to read line from stdin
use std::io::{BufRead, BufReader};
/// This function read line from stdin and return it as a String
/// It will panic if it fails to read line with a message "Failed to read line"
/// #Examples:
/// ....
/// let input = read_stdin();
/// '''
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}