pub enum CreditCardVerificationError {
    InvalidCreditCard,
    InvalidDigitsFound,
    InvalidLength,
}

fn m_mul(a: u32, b: u32, modulo: u32) -> u32 {
    let sum = a * b;
    if sum > modulo {
        sum - modulo
    } else {
        sum
    }
}

pub fn verify_credit_card(credit_card_number: &str) -> Result<(), CreditCardVerificationError> {
    let stripped_input = credit_card_number.replace(" ", "");

    if !stripped_input.chars().all(char::is_numeric) {
        return Err(CreditCardVerificationError::InvalidDigitsFound);
    }

    if stripped_input.len() != 16 {
        return Err(CreditCardVerificationError::InvalidLength);
    }

    let integers: Vec<u32> = stripped_input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let odd = integers.iter().skip(1).step_by(2).map(|d| *d as u32);
    let even = integers.iter().step_by(2).map(|&d| m_mul(2, d, 9));
    let both: u32 = itertools::interleave(even, odd).sum();

    if both % 10 == 0 {
        Ok(())
    } else {
        Err(CreditCardVerificationError::InvalidCreditCard)
    }
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
