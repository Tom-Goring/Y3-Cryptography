pub enum ISBNVerificationError {
    InvalidDigitCount,
    NonValidISBN,
    InvalidDigitsFound,
}

pub fn verify_isbn(isbn: &str) -> Result<(), ISBNVerificationError> {
    let mut error_digit: i32 = -1;

    let mut stripped_isbn = isbn.replace("-", "");

    if stripped_isbn.len() != 10 {
        return Err(ISBNVerificationError::InvalidDigitCount);
    }

    if stripped_isbn.ends_with('X') {
        error_digit = 10;
        stripped_isbn.pop();
    }

    if !stripped_isbn.chars().all(char::is_numeric) {
        return Err(ISBNVerificationError::InvalidDigitsFound);
    }

    if error_digit == -1 {
        error_digit = stripped_isbn.pop().unwrap().to_digit(10).unwrap() as i32;
    }

    if stripped_isbn
        .chars()
        .enumerate()
        .map(|(i, c)| (i as i32 + 1) * c.to_digit(10).unwrap() as i32)
        .sum::<i32>()
        % 11
        != error_digit
    {
        Err(ISBNVerificationError::NonValidISBN)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn isbn_verification_success() {
        let valid_isbn_list = vec![
            "99921-58-10-7",
            "960-425-059-0",
            "1-84356-028-3",
            "0-943396-04-2",
            "0-2-33-56131-5",
            "0-9752298-0-X",
        ];

        valid_isbn_list
            .iter()
            .for_each(|&isbn| assert!(verify_isbn(&String::from(isbn)).is_ok()));
    }

    #[test]
    pub fn isbn_verification_failure() {
        let invalid_isbn_list = vec!["0-2-83-56131-5"];

        invalid_isbn_list
            .iter()
            .for_each(|&isbn| assert!(verify_isbn(&String::from(isbn)).is_err()));
    }

    #[test]
    pub fn isbn_verification_digit_count() {
        let invalid_isbn = "99921-58-10-756";

        assert!(verify_isbn(&String::from(invalid_isbn)).is_err()); // TODO: match error
    }

    #[test]
    pub fn isbn_verification_invalid_digits() {
        let invalid_isbn = "9a92b-5c-10-7d6";

        assert!(verify_isbn(&String::from(invalid_isbn)).is_err()); // TODO: match error
    }
}
