//! Iterator over a comma-seperated string, ignoring any commas inside quotes
//!
//! # Example
//!
//! ```rust
//! # fn main() {
//! # use comma_separated::CommaSeparatedIterator;
//! let input = r#"foo, "bar", 'quoted, part'"#;
//! let iterator = CommaSeparatedIterator::new(input);
//! assert_eq!(vec!["foo", "\"bar\"", "'quoted, part'"], iterator.collect::<Vec<_>>());
//! # }
//! ```

#[derive(Copy, Clone)]
enum CommaSeparatedIteratorState {
    /// Start of string or after a ',' (including whitespace)
    Default,
    /// Inside a quote
    Quoted(Quote),
    /// After escape character inside quote
    QuotedPair(Quote),
    /// Non quoted part
    Token,
    /// After closing quote
    PostAmbleForQuoted,
}

#[derive(Copy, Clone)]
enum Quote {
    Single,
    Double,
}

pub struct CommaSeparatedIterator<'a> {
    /// target
    target: &'a str,
    /// iterator
    char_indices: std::str::CharIndices<'a>,
    /// current scanner state
    state: CommaSeparatedIteratorState,
    /// start position of the last token found
    s: usize,
}

impl<'a> CommaSeparatedIterator<'a> {
    /// Create a new iterator, splitting the input into comma-seperated parts with handling of quoted segments
    pub fn new(target: &'a str) -> Self {
        Self {
            target,
            char_indices: target.char_indices(),
            state: CommaSeparatedIteratorState::Default,
            s: 0,
        }
    }
}

impl<'a> Iterator for CommaSeparatedIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        for (i, c) in &mut self.char_indices {
            let (next, next_state) = match (self.state, c) {
                (CommaSeparatedIteratorState::Default, '"') => {
                    self.s = i;
                    (None, CommaSeparatedIteratorState::Quoted(Quote::Double))
                }
                (CommaSeparatedIteratorState::Default, '\'') => {
                    self.s = i;
                    (None, CommaSeparatedIteratorState::Quoted(Quote::Single))
                }
                (CommaSeparatedIteratorState::Default, ' ' | '\t') => {
                    (None, CommaSeparatedIteratorState::Default)
                }
                (CommaSeparatedIteratorState::Default, ',') => (
                    Some(Some(&self.target[i..i])),
                    CommaSeparatedIteratorState::Default,
                ),
                (CommaSeparatedIteratorState::Default, _) => {
                    self.s = i;
                    (None, CommaSeparatedIteratorState::Token)
                }
                (CommaSeparatedIteratorState::Quoted(Quote::Double), '"')
                | (CommaSeparatedIteratorState::Quoted(Quote::Single), '\'') => (
                    Some(Some(&self.target[self.s..i + 1])),
                    CommaSeparatedIteratorState::PostAmbleForQuoted,
                ),
                (CommaSeparatedIteratorState::Quoted(quote), '\\') => {
                    (None, CommaSeparatedIteratorState::QuotedPair(quote))
                }
                (CommaSeparatedIteratorState::QuotedPair(quote), _) => {
                    (None, CommaSeparatedIteratorState::Quoted(quote))
                }
                (CommaSeparatedIteratorState::Token, ',') => (
                    Some(Some(&self.target[self.s..i])),
                    CommaSeparatedIteratorState::Default,
                ),
                (CommaSeparatedIteratorState::PostAmbleForQuoted, ',') => {
                    (None, CommaSeparatedIteratorState::Default)
                }
                (current_state, _) => (None, current_state),
            };
            self.state = next_state;
            if let Some(next) = next {
                return next;
            }
        }
        match self.state {
            CommaSeparatedIteratorState::Default
            | CommaSeparatedIteratorState::PostAmbleForQuoted => None,
            CommaSeparatedIteratorState::Quoted(_)
            | CommaSeparatedIteratorState::QuotedPair(_)
            | CommaSeparatedIteratorState::Token => {
                self.state = CommaSeparatedIteratorState::Default;
                Some(&self.target[self.s..])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CommaSeparatedIterator;

    #[test]
    fn test_comma_separated_iterator() {
        assert_eq!(
            vec!["abc", "def", "ghi", "jkl ", "mno", "pqr"],
            CommaSeparatedIterator::new("abc,def, ghi,\tjkl , mno,\tpqr").collect::<Vec<&str>>()
        );
        assert_eq!(
            vec![
                "abc",
                "\"def\"",
                "\"ghi\"",
                "\"jkl\"",
                "\"mno\"",
                "pqr",
                "\"abc, def\"",
                "foo",
                "\" foo\"",
                "',foo'",
                "\"fo'o\"",
            ],
            CommaSeparatedIterator::new(
                "abc,\"def\", \"ghi\",\t\"jkl\" , \"mno\",\tpqr, \"abc, def\",  foo, \" foo\", ',foo', \"fo'o\""
            )
                .collect::<Vec<&str>>()
        );
    }
}
