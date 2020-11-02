use std::char;
use std::ops::Add;

const D7: [i32; 6] = [4, 10, 9, 2, 1, 7];
const D8: [i32; 6] = [7, 8, 7, 1, 9, 6];
const D9: [i32; 6] = [9, 1, 7, 8, 7, 7];
const D10 : [i32; 6] = [1, 2, 9, 10, 4, 1];

pub fn calculate_digit(weights: &[i32], input: &[i32]) -> Result<i32, ()> {
    if input.len() != 6 { return Err(()) }
    if weights.len() != 6 { return Err(()) }

    Ok(weights.iter().zip(input.iter()).map(|(weight, digit)| (weight * digit)).sum::<i32>() % 11)
}

pub fn calculate_digits<'a>(weights: &[&[i32]], input: &[i32]) -> Result<String, String> {
    if input.len() != 6 { return Err("Input length too small".into()) }

    let check_digits: Result<String, _> = weights.iter().map(|weights| {
        char::from_digit(calculate_digit(*weights, input).unwrap() as u32, 10).ok_or("Unusable number")
    }).collect();

    match check_digits {
        Ok(digits) => {
            let input_string = input.iter().map(|d| char::from_digit(*d as u32, 10).unwrap()).collect::<String>();
            let output = input_string.add(&digits);
            Ok(output)
        },
        Err(error) => {
            Err(format!("{}", error))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn check_digits_successful() {
        let weights = [&D7[..], &D8[..], &D9[..], &D10[..]];

        let inputs = vec![
            [0,0,0,0,0,1],
            [0,0,0,0,0,2],
            [0,0,0,0,1,0],
            [0,0,0,0,1,1]
        ];

        let results = vec![
            "0000017671",
            "0000023132",
            "0000101974",
            "0000118435"
        ];

        inputs.iter().enumerate().for_each(|(i, _)| assert_eq!(&calculate_digits(&weights[..], &inputs[i][..]).unwrap(), results[i]));
    }

    #[test]
    pub fn check_digits_failure() {
        let weights = [&D7[..], &D8[..], &D9[..], &D10[..]];
        let input = [0,0,0,0,0,3];

        calculate_digits(&weights, &input[..]).unwrap_err();
    }
}
