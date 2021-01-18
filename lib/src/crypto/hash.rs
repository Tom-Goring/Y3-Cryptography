use sha1::Sha1;

pub fn sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    hasher.digest().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sha() {
        let input = "hello";
        let result = sha1(input);
        assert_eq!(result, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    }
}
