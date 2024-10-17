//! This crate provides utilities for splitting strings based on a delimiter.
//!
//!

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]

/// Structure to support splitting a string based on a delimiter.
///
/// The `StrSplit` struct holds the state necessary to split a string
/// into substrings based on a specified delimiter.
///
/// # Examples
///
///
/// let splitter = StrSplit::new("hello world", " ");
/// // Use the splitter...
///
/// # Fields
///
/// * `remainder` - The part of the string that has not been split yet.
/// * `delimiter` - The delimiter used to split the string.
pub struct StrSplit<'a> {
    /// The part of the string that has not been split yet.
    remainder: Option<&'a str>,
    /// The delimiter used to split the string.
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    /// Creates a new `StrSplit` with the given string and delimiter.
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[test]
fn it_works() {
    let splitter = StrSplit::new("a b c d e", " ");
    let results: Vec<&str> = splitter.collect();
    assert_eq!(results, vec!["a", "b", "c", "d", "e"]);
}
