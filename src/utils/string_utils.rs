// Tokenizer that handles single and double quotes.
// - Characters inside single quotes are treated literally.
// - Characters inside double quotes are treated literally (no special handling/expansion).
// - Whitespace outside of quotes delimits tokens and consecutive whitespace collapses.
// - Adjacent quoted/unquoted segments are concatenated into the same token.
// - Empty quotes ('', "") are ignored (i.e., contribute nothing but can concatenate neighbors).
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;

    for ch in input.chars() {
        match ch {
            '\'' if !in_double => {
                // toggle single quote context (do not include the quote itself)
                in_single = !in_single;
            }
            '"' if !in_single => {
                // toggle double quote context (do not include the quote itself)
                in_double = !in_double;
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                // collapse consecutive whitespace outside quotes by skipping
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
