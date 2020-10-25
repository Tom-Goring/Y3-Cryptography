pub enum ISBNVerificationError {
    InvalidDigitCount,
    NonValidISBN,
    InvalidDigitsFound,
}

pub enum CreditCardVerificationError {
    InvalidCreditCard,
    InvalidDigitsFound,
    InvalidLength,
}

// TODO: Add support for final digit X

pub fn verify_isbn(isbn: &String) -> Result<(), ISBNVerificationError> {
    let mut error_digit: i32 = -1;

    let mut stripped_isbn = isbn.replace("-", "");

    if stripped_isbn.len() != 10 {
        return Err(ISBNVerificationError::InvalidDigitCount);
    }

    if stripped_isbn.chars().last().unwrap() == 'X' {
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

pub fn verify_credit_card(credit_card_number: &String) -> Result<(), CreditCardVerificationError> {
    let stripped_input = credit_card_number.replace(" ", "");

    if !stripped_input.chars().all(char::is_numeric) {
        return Err(CreditCardVerificationError::InvalidDigitsFound);
    }

    if stripped_input.len() != 16 {
        return Err(CreditCardVerificationError::InvalidLength);
    }

    if stripped_input
        .chars()
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 0 {
                let mut x = 2 * d.to_digit(10).unwrap();
                if x >= 10 {
                    x -= 9;
                }
                x
            } else {
                d.to_digit(10).unwrap()
            }
        })
        .sum::<u32>()
        % 10
        == 0
    {
        return Ok(());
    }

    Err(CreditCardVerificationError::InvalidCreditCard)
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

    #[test]
    pub fn credit_verification_success() {
        let valid_credit_no_list = [
            "0980108875738176",
            "9761915824794746",
            "3644685256981147",
            "6921 9308 3585 6403",
        ];

        valid_credit_no_list
            .iter()
            .for_each(|&credit_no| assert!(verify_credit_card(&String::from(credit_no)).is_ok()));
    }

    #[test]
    pub fn credit_verification_digit_count() {
        let invalid_no = "692193083585640"; // 15 digits

        assert!(verify_credit_card(&String::from(invalid_no)).is_err());
    }

    #[test]
    pub fn credit_verification_alphanumeric() {
        let invalid_no = "692193083585640A";

        assert!(verify_credit_card(&String::from(invalid_no)).is_err());
    }
}
