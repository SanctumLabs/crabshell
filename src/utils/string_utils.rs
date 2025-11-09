// Tokenizer that handles single and double quotes, and backslash escaping outside quotes.
// - Characters inside single quotes are treated literally.
// - Characters inside double quotes are treated literally (no special handling/expansion).
// - A non-quoted backslash (\\) escapes the next character, preserving it literally (including whitespace/quotes/backslash).
// - Whitespace outside of quotes delimits tokens and consecutive whitespace collapses.
// - Adjacent quoted/unquoted segments are concatenated into the same token.
// - Empty quotes ('', "") are ignored (i.e., contribute nothing but can concatenate neighbors).
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;

    let mut iter = input.chars().peekable();
    while let Some(ch) = iter.next() {
        if in_single {
            if ch == '\'' {
                in_single = false;
            } else {
                current.push(ch);
            }
            continue;
        }
        if in_double {
            if ch == '"' {
                in_double = false;
            } else {
                current.push(ch);
            }
            continue;
        }

        // Outside quotes
        match ch {
            '\\' => {
                // Escape next char if present, otherwise treat trailing backslash as literal
                if let Some(next_ch) = iter.next() {
                    current.push(next_ch);
                } else {
                    current.push('\\');
                }
            }
            '\'' => {
                in_single = true;
            }
            '"' => {
                in_double = true;
            }
            c if c.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                // collapse consecutive whitespace by skipping
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
