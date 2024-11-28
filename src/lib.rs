//! Iterator over a comma-seperated string, ignoring any commas inside quotes
//!
//! # Example
//!
//! ```rust
//! # use comma_separated::CommaSeparatedIterator;
//! # fn main() {
//! let input = r#"foo,"bar",'quoted, part'"#;
//! let iterator = CommaSeparatedIterator::new(input);
//! assert_eq!(vec!["foo", "\"bar\"", "'quoted, part'"], iterator.collect::<Vec<_>>());
//! # }
//! ```

#[derive(Copy, Clone)]
enum CommaSeparatedIteratorState {
    /// Non quoted part
    Default,
    /// Inside a quote
    Quoted(Quote),
    /// After escape character inside quote
    QuotedEscape(Quote),
}

#[derive(Copy, Clone)]
enum Quote {
    Single,
    Double,
}

pub struct CommaSeparatedIterator<'a> {
    remaining: &'a str,
}

impl<'a> CommaSeparatedIterator<'a> {
    /// Create a new iterator, splitting the input into comma-seperated parts with handling of quoted segments
    pub fn new(text: &'a str) -> Self {
        Self { remaining: text }
    }
}

impl<'a> Iterator for CommaSeparatedIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        let mut state = CommaSeparatedIteratorState::Default;
        let mut char_indices = self.remaining.char_indices();

        for (i, c) in &mut char_indices {
            state = match (state, c) {
                (CommaSeparatedIteratorState::Default, '"') => {
                    CommaSeparatedIteratorState::Quoted(Quote::Double)
                }
                (CommaSeparatedIteratorState::Default, '\'') => {
                    CommaSeparatedIteratorState::Quoted(Quote::Single)
                }
                (CommaSeparatedIteratorState::Quoted(Quote::Double), '"')
                | (CommaSeparatedIteratorState::Quoted(Quote::Single), '\'') => {
                    CommaSeparatedIteratorState::Default
                }
                (CommaSeparatedIteratorState::Quoted(quote), '\\') => {
                    CommaSeparatedIteratorState::QuotedEscape(quote)
                }
                (CommaSeparatedIteratorState::Quoted(quote), _) => {
                    CommaSeparatedIteratorState::Quoted(quote)
                }
                (CommaSeparatedIteratorState::QuotedEscape(quote), _) => {
                    CommaSeparatedIteratorState::Quoted(quote)
                }
                (CommaSeparatedIteratorState::Default, ',') => {
                    let result = &self.remaining[0..i];
                    self.remaining = &self.remaining[i + 1..];
                    return Some(result);
                }
                (CommaSeparatedIteratorState::Default, _) => CommaSeparatedIteratorState::Default,
            };
        }
        let result = self.remaining;
        self.remaining = "";
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::CommaSeparatedIterator;

    #[test]
    fn test_comma_separated_iterator() {
        assert_eq!(
            vec!["abc", "def", " ghi", "\tjkl ", " mno", "\tpqr"],
            CommaSeparatedIterator::new("abc,def, ghi,\tjkl , mno,\tpqr").collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![
                r#""abc,def""#,
                " \"ghi\"",
                "\"jkl\" ",
                " \"mno\"",
                "pqr",
                " \"abc, def\"",
                " foo",
                " \" foo\"",
                " ',foo'",
                " \"fo'o\"",
            ],
            CommaSeparatedIterator::new(
                r#""abc,def", "ghi","jkl" , "mno",pqr, "abc, def", foo, " foo", ',foo', "fo'o""#
            )
                .collect::<Vec<&str>>()
        );
    }
}
