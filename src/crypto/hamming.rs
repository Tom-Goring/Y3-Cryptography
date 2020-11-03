use std::char;
use std::ops::Add;

pub const D7: [u32; 6] = [4, 10, 9, 2, 1, 7];
pub const D8: [u32; 6] = [7, 8, 7, 1, 9, 6];
pub const D9: [u32; 6] = [9, 1, 7, 8, 7, 7];
pub const D10: [u32; 6] = [1, 2, 9, 10, 4, 1];

pub fn calculate_digit(weights: &[u32], input: &[u32]) -> Result<u32, ()> {
    if input.len() != 6 {
        return Err(());
    }
    if weights.len() != 6 {
        return Err(());
    }

    Ok(weights
        .iter()
        .zip(input.iter())
        .map(|(weight, digit)| (weight * digit))
        .sum::<u32>()
        % 11)
}

pub fn calculate_digits(weights: &[&[u32]], input: &String) -> Result<String, String> {
    if input.len() != 6 {
        return Err("Input length too small".into());
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
                    calculate_digit(*weights, &digit_vec[..]).unwrap() as u32,
                    10,
                )
                .ok_or("Unusable number".into())
            })
            .collect(),
        Err(error) => Err(error.into()),
    };

    match check_digits {
        Ok(digits) => Ok(input.clone().add(&digits)),
        Err(error) => Err(format!("{}", error)),
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
                &calculate_digits(&weights[..], inputs[i].into()).unwrap(),
                results[i]
            )
        });
    }

    #[test]
    pub fn check_digits_failure() {
        let weights = [&D7[..], &D8[..], &D9[..], &D10[..]];
        let input = "000003";

        calculate_digits(&weights, input.into()).unwrap_err();
    }
}
