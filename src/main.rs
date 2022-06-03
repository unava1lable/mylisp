fn main() {
    println!("{:?}", tokenize("(begin (define r 10) (* pi (* r r)))"))
}

fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut prev = String::new();

    for c in s.chars() {
        match c {
            ' ' => {
                if !prev.is_empty() {
                    tokens.push(prev.clone());
                    prev = String::new();
                }
            }
            c @ ('(' | ')') => {
                if !prev.is_empty() {
                    tokens.push(prev.clone());
                    prev = String::new();
                }
                tokens.push(c.to_string());
            }
            _ => {
                prev.push(c);
            }
        }
    }
    tokens
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(vec!["(".to_string(), ")".to_string()], tokenize("()"));
}