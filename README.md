# Moved to https://codeberg.org/icewind/comma-separated

# Comma-separated

Iterator over a comma-seperated string, ignoring any commas inside quotes

```rust
use comma_separated::CommaSeparatedIterator;

fn main() {
    let input = r#"foo, "bar", 'quoted, part'"#;
    let iterator = CommaSeparatedIterator::new(input);
    assert_eq!(vec!["foo", "\"bar\"", "'quoted, part'"], iterator.collect::<Vec<_>>());
}
```
