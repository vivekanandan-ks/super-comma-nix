use std::env;

pub fn parse_nflags_str(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    for c in input.chars() {
        match c {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

pub fn get_super_flags(raw_args: &[String]) -> Vec<String> {
    let mut super_flags = Vec::new();
    if let Ok(env_sflags) = env::var("SUPER_COMMA_FLAGS") {
        super_flags.extend(parse_nflags_str(&env_sflags));
    }
    raw_args.iter().cloned().chain(super_flags).collect()
}
