//! This crate provides utilities for splitting strings based on a delimiter.
//!
//!

//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
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
pub struct StrSplit<'haystack, D> {
    /// The part of the string that has not been split yet.
    remainder: Option<&'haystack str>,
    /// The delimiter used to split the string.
    delimiter: D,
}

// str -> [char]
// &str -> &[char]
// String -> Vec<char>
//
// String -> &str (cheap -- AsRef)
// &str -> String (expensive -- memcopy)


impl<'haystack, D> StrSplit<'haystack, D> {
    /// Creates a new `StrSplit` with the given string and delimiter.
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize,usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| {
            (start, start + self.len())
        })
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
        .find(|(_, c)| c == self)
        .map(|(delim_start, _)| (delim_start, delim_start + self.len_utf8()))
    }
}

#[allow(dead_code)]
pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let splitter = StrSplit::new("a b c d e", " ");
    let results: Vec<&str> = splitter.collect();
    assert_eq!(results, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}