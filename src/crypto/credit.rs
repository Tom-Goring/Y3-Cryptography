pub enum CreditCardVerificationError {
    InvalidCreditCard,
    InvalidDigitsFound,
    InvalidLength,
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
