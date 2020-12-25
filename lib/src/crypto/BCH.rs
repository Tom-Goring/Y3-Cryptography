#![allow(dead_code)]
use super::hamming::*;
use crate::crypto::modular::Modular;
use std::char;

#[derive(Debug)]
pub enum CodingResult {
    EncodingError(String),
    InputError(String),
    SingleError(String, i32, i32),
    DoubleError(String, (i32, i32), (i32, i32)),
    TripleError(String),
}

pub fn encode_bch(string: &str) -> Result<String, CodingResult> {
    let weights = [&D7[..], &D8[..], &D9[..], &D10[..]];
    match calculate_digits(&weights, string, 6) {
        Ok(digits) => Ok(digits),
        Err(error) => Err(CodingResult::EncodingError(error)),
    }
}

pub fn decode_bch(string: &str) -> Result<String, CodingResult> {
    if string.len() != 10 {
        return Err(CodingResult::InputError("Input is of wrong length".into()));
    }

    let integers: Vec<i32> = string
        .chars()
        .map(|c| {
            c.to_digit(10).map_or_else(
                || {
                    Err(CodingResult::InputError(
                        "String contains a non numeric character".into(),
                    ))
                },
                |i| Ok(i as i32),
            )
        })
        .collect::<Result<Vec<i32>, _>>()?;

    let syndrome_vector = (0..=3)
        .into_iter()
        .map(|digit| {
            Modular::new(
                integers
                    .iter()
                    .zip(0..)
                    .map(|(&x, i)| x * (i as i32 + 1).pow(digit))
                    .sum::<i32>(),
                11,
            )
        })
        .collect::<Vec<Modular>>();

    if syndrome_vector == [0, 0, 0, 0] {
        return Ok(string.into());
    }

    let p = syndrome_vector[1].pow(2) - syndrome_vector[0] * syndrome_vector[2];
    let q = syndrome_vector[0] * syndrome_vector[3] - syndrome_vector[1] * syndrome_vector[2];
    let r = syndrome_vector[2].pow(2) - syndrome_vector[1] * syndrome_vector[3];

    return if p.value() + q.value() + r.value() == 0 {
        let position = match syndrome_vector[1].try_div(syndrome_vector[0]) {
            Ok(res) => res,
            Err(err) => return Err(CodingResult::TripleError(err.into())),
        };

        let magnitude = syndrome_vector[0];

        if position == 0 {
            return Err(CodingResult::TripleError(
                "Error was detected at position zero!".into(),
            ));
        }

        let correct_digit = Modular::new(integers[position.value() as usize - 1], 11) - magnitude;

        let mut new_integers = integers.clone();
        new_integers[position.value() as usize - 1] = correct_digit.value();

        let output = new_integers
            .iter()
            .map(|&d| char::from_digit(d as u32, 10).unwrap())
            .collect::<String>();

        Err(CodingResult::SingleError(
            output,
            position.value(),
            magnitude.value(),
        ))
    } else {
        let pre_root = Modular::new(q.value().pow(2) - (4 * p.value() * r.value()), 11);

        let root = match pre_root.sqrt() {
            Ok(root) => root,
            Err(_) => {
                return Err(CodingResult::TripleError(
                    "Q^2 - 4PR did not have a valid root!".into(),
                ))
            }
        };

        let position1 = match (root - q).try_div(p * Modular::new(2, 11)) {
            Ok(res) => res,
            Err(err) => return Err(CodingResult::TripleError(err.into())),
        };

        let position2 = match (Modular::new(0, 11) - q - root).try_div(p * Modular::new(2, 11)) {
            Ok(res) => res,
            Err(err) => return Err(CodingResult::TripleError(err.into())),
        };

        if position1 == 0 || position2 == 0 {
            return Err(CodingResult::TripleError(
                "Error was detected at position zero!".into(),
            ));
        }

        let magnitude2 = match (position1 * syndrome_vector[0] - syndrome_vector[1])
            .try_div(position1 - position2)
        {
            Ok(res) => res,
            Err(err) => return Err(CodingResult::TripleError(err.into())),
        };

        let magnitude1 = syndrome_vector[0] - magnitude2;

        let mut new_integers = integers.clone();

        let corrected1 = Modular::new(integers[position1.value() as usize - 1], 11)
            + (Modular::new(11, 11) - magnitude1);
        let corrected2 = Modular::new(integers[position2.value() as usize - 1], 11)
            + (Modular::new(11, 11) - magnitude2);

        new_integers[position1.value() as usize - 1] = corrected1.value();
        new_integers[position2.value() as usize - 1] = corrected2.value();

        let output: Result<String, CodingResult> = new_integers
            .iter()
            .map(|&d| {
                char::from_digit(d as u32, 10).map_or_else(
                    || {
                        Err(CodingResult::TripleError(
                            "Corrected a digit into 10".into(),
                        ))
                    },
                    |c| Ok(c),
                )
            })
            .collect();

        match output {
            Ok(output) => Err(CodingResult::DoubleError(
                output,
                (position1.value(), position2.value()),
                (magnitude1.value(), magnitude2.value()),
            )),
            Err(error) => Err(error),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn bch_success() {
        let x = decode_bch("3745195876").unwrap();
        assert_eq!(&x, "3745195876");
    }

    #[test]
    pub fn bch_single_error() {
        let y = decode_bch("3945195876").unwrap_err();
        match y {
            CodingResult::SingleError(output, pos, mag) => {
                assert_eq!((output.as_str(), pos, mag), ("3745195876", 2, 2))
            }
            err => panic!("Wrong error type returned. Error was: {:?}", err),
        }
    }

    #[test]
    pub fn bch_double_error() {
        let inputs = vec![
            ("3715195076", "3745195876", (8, 3), (3, 8)),
            ("0743195876", "3745195876", (4, 1), (9, 8)),
            ("3745195840", "3745195876", (10, 9), (5, 8)),
            ("8745105876", "3745195876", (6, 1), (2, 5)),
            ("3745102876", "3745195876", (6, 7), (2, 8)),
            ("1145195876", "3745195876", (1, 2), (9, 5)),
            ("3745191976", "3745195876", (8, 7), (1, 7)),
            ("3745190872", "3745195876", (7, 10), (6, 7)),
        ];

        inputs.iter().for_each(|row| {
            let result = decode_bch(row.0).unwrap_err();
            match result {
                CodingResult::DoubleError(output, pos, mag) => {
                    assert_eq!(row.1, output);
                    assert_eq!(row.2, (pos));
                    assert_eq!(row.3, (mag));
                }
                _ => panic!("Wrong error type returned"),
            }
        });
    }

    #[test]
    pub fn bch_triple_error() {
        let inputs = vec![
            "1115195876",
            "2745795878",
            "3742102896",
            "1115195876",
            "3121195876",
            "3121195876",
            "1135694766",
            "0888888074",
            "5614216009",
            "9990909923",
            "1836703776",
            "9885980731",
        ];

        inputs.iter().for_each(|row| {
            let result = decode_bch(row).unwrap_err();
            match result {
                CodingResult::TripleError(_) => {}
                _ => panic!("Wrong error type returned"),
            }
        });
    }
}
