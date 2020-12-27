use std::fmt::Formatter;

/// Weights generated using a galois field as described in the optional course materials.
const WEIGHTS: [[u32; 6]; 4] = [
    [4, 10, 9, 2, 1, 7],
    [7, 8, 7, 1, 9, 6],
    [9, 1, 7, 8, 7, 7],
    [1, 2, 9, 10, 4, 1],
];

const SYNDROME_WEIGHTS: [[u32; 10]; 4] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    [1, 4, 9, 5, 3, 3, 5, 9, 4, 1],
    [1, 8, 5, 9, 4, 7, 2, 6, 3, 10],
];

#[derive(Debug, Copy, Clone)]
pub enum HammingError {
    InvalidDigit,
    UnusableNumber,
    InvalidLength(usize, usize),
}

impl std::fmt::Display for HammingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HammingError::InvalidDigit => {
                write!(f, "Hamming code error: Invalid digit received in input")
            }
            HammingError::UnusableNumber => {
                write!(f, "Hamming code error: Input is unusable")
            }
            HammingError::InvalidLength(required, actual) => {
                write!(
                    f,
                    "Hamming code error: Input is of wrong length - given input is of \
                length {} but a length of {} is required!",
                    actual, required
                )
            }
        }
    }
}

/// Calculates the check digits for a given input. An HammingCode error is returned if any of the
/// check digits are 10 or if an input character is non-numeric. The result is not the entire
/// string, just the resulting check digits.
pub fn calculate_hamming_check_digits(input: &str) -> Result<String, HammingError> {
    if input.len() != 6 {
        return Err(HammingError::InvalidLength(6, input.len()));
    }
    match input
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    {
        Some(digits) => match WEIGHTS
            .iter()
            .map(|weights| {
                weights
                    .iter()
                    .zip(digits.iter())
                    .map(|(weight, digit)| weight * digit)
                    .sum::<u32>()
                    % 11
            })
            .map(|check_digit| std::char::from_digit(check_digit, 10))
            .collect::<Option<String>>()
        {
            Some(check_digits) => Ok(check_digits),
            None => Err(HammingError::UnusableNumber),
        },
        None => Err(HammingError::InvalidDigit),
    }
}

pub fn generate_syndromes(input: &str) -> Result<Vec<u32>, HammingError> {
    if input.len() != 10 {
        return Err(HammingError::InvalidLength(10, input.len()));
    }

    match input
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    {
        Some(digits) => Ok(SYNDROME_WEIGHTS
            .iter()
            .map(|weights| {
                weights
                    .iter()
                    .zip(digits.iter())
                    .map(|(weight, digit)| weight * digit)
                    .sum::<u32>()
                    % 11
            })
            .collect::<Vec<u32>>()),
        None => Err(HammingError::InvalidDigit),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn hamming_check_digits_success() {
        let inputs = ["000001", "000002", "000010", "000011"];
        let results = ["7671", "3132", "1974", "8435"];
        for (input, proper) in inputs.iter().zip(results.iter()) {
            let result = calculate_hamming_check_digits(input);
            match result.ok() {
                Some(output) => assert_eq!(output, String::from(*proper)),
                None => panic!(),
            }
        }
    }

    #[test]
    pub fn hamming_check_digits_unusable_number() {
        let result = calculate_hamming_check_digits("000003");
        if result.is_ok() {
            panic!()
        }
    }

    #[test]
    pub fn hamming_check_digits_invalid_characters() {
        let result = calculate_hamming_check_digits("flnefk");
        if result.is_ok() {
            panic!()
        }
    }

    #[test]
    pub fn syndrome_vector_generation_success() {
        let inputs = ["0000118435", "8899880747"];
        let results = [[0, 0, 0, 0], [2,7,3,3]];
        for (input, proper) in inputs.iter().zip(results.iter()) {
            let result = generate_syndromes(input);
            match result.clone().ok() {
                Some(output) => assert_eq!(output, proper),
                None => panic!(println!("{:?}", result.unwrap())),
            }
        }
    }
}
