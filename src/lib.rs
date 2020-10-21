use crate::ISBNVerificationError::{NonValidISBN, InvalidDigitCount, InvalidDigitsFound};

pub enum ISBNVerificationError {
    InvalidDigitCount,
    NonValidISBN,
    InvalidDigitsFound,
}

pub fn verify_isbn(isbn: &String) -> Result<(), ISBNVerificationError> {
    let mut stripped_isbn = isbn.replace("-", "");

    if stripped_isbn.len() != 10 {
        return Err(InvalidDigitCount)
    }

    if !stripped_isbn.chars().all(char::is_numeric) {
        return Err(InvalidDigitsFound)
    }

    let error_digit = stripped_isbn.pop().unwrap().to_digit(10).unwrap();

    if stripped_isbn
        .chars()
        .enumerate()
        .map(|(i, c)| (i as u32 + 1) * c.to_digit(10).unwrap())
        .sum::<u32>() % 11 != error_digit {
        Err(NonValidISBN)
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
            "0-2-33-56131-5"
        ];

        valid_isbn_list.iter().for_each(|&isbn| assert!(verify_isbn(&String::from(isbn)).is_ok()));
    }

    #[test]
    pub fn isbn_verification_failure() {
        let invalid_isbn_list = vec![
            "0-2-83-56131-5",
        ];

        invalid_isbn_list.iter().for_each(|&isbn| assert!(verify_isbn(&String::from(isbn)).is_err()));
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
