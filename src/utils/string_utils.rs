// Simple tokenizer that handles single quotes. Characters inside single quotes are treated literally.
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;

    for ch in input.chars() {
        match ch {
            '\'' => {
                in_single = !in_single;
            }
            c if c.is_whitespace() && !in_single => {
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
