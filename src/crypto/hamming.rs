use std::char;
use std::ops::Add;

pub const D7: [u32; 6] = [4, 10, 9, 2, 1, 7];
pub const D8: [u32; 6] = [7, 8, 7, 1, 9, 6];
pub const D9: [u32; 6] = [9, 1, 7, 8, 7, 7];
pub const D10: [u32; 6] = [1, 2, 9, 10, 4, 1];

pub const S1: [u32; 10] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
pub const S2: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
pub const S3: [u32; 10] = [1, 4, 9, 5, 3, 3, 5, 9, 4, 1];
pub const S4: [u32; 10] = [1, 8, 5, 9, 4, 7, 2, 6, 3, 10];

pub fn calculate_digit(weights: &[u32], input: &[u32], length: usize) -> Result<u32, &'static str> {
    if input.len() != length {
        println!("Input length does not equal length.");
        return Err("Input length does not equal length.");
    }
    if weights.len() != length {
        println!("Weights length does not equal length.");
        return Err("Weights length does not equal length.");
    }

    Ok(weights
        .iter()
        .zip(input.iter())
        .map(|(weight, digit)| (weight * digit))
        .sum::<u32>()
        % 11)
}

pub fn calculate_digits(weights: &[&[u32]], input: &str, length: usize) -> Result<String, String> {
    if input.len() != length {
        return Err(format!(
            "Input length too small - should be {} not {}!",
            length,
            input.len()
        ));
    }

    let input_as_digits: Result<Vec<u32>, _> = input
        .chars()
        .map(|c| c.to_digit(10).ok_or("Invalid Digit"))
        .collect();

    let check_digits: Result<String, String> = match input_as_digits {
        Ok(digit_vec) => weights
            .iter()
            .map(|weights| {
                char::from_digit(
                    calculate_digit(*weights, &digit_vec[..], length).unwrap() as u32,
                    10,
                )
                .ok_or_else(|| format!("Inputted string cannot be encoded using BCH(10,6)"))
            })
            .collect(),
        Err(error) => Err(error.into()),
    };

    match check_digits {
        Ok(digits) => Ok(String::from(input).add(&digits)),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn check_digits_successful() {
        let weights = [&D7[..], &D8[..], &D9[..], &D10[..]];

        let inputs = vec!["000001", "000002", "000010", "000011"];

        let results = vec!["0000017671", "0000023132", "0000101974", "0000118435"];

        inputs.iter().enumerate().for_each(|(i, _)| {
            assert_eq!(
                &calculate_digits(&weights[..], &String::from(inputs[i]), 6).unwrap(),
                results[i]
            )
        });
    }

    #[test]
    pub fn check_digits_failure() {
        let weights = [&D7[..], &D8[..], &D9[..], &D10[..]];
        let input = "000003";

        calculate_digits(&weights, &String::from(input), 6).unwrap_err();
    }

    #[test]
    pub fn check_syndrome_generation_success() {
        let weights = [&S1[..], &S2[..], &S3[..], &S4[..]];
        let input = "0000118435";

        let digits = calculate_digits(&weights, &String::from(input), 10).unwrap();

        assert_eq!(digits, String::from("00001184350000"));

        let input = "8899880747";

        let digits = calculate_digits(&weights, &String::from(input), 10).unwrap();

        assert_eq!(digits, String::from("88998807472733"));
    }
}
