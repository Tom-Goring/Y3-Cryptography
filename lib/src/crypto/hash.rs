use rl_crypto::digest::Digest;
use rl_crypto::sha1::Sha1;

pub fn sha1(input: &str) -> String {
    let mut sha = Sha1::new();
    sha.input_str(input);
    sha.result_str()
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
