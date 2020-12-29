#![allow(dead_code)]
use super::hamming::*;
use super::modular::*;
use BCHError::*;
use TripleErrorReason::*;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BCHError {
    InvalidInput(HammingError),
    SingleError(Vec<u32>, u32, u32),
    DoubleError(Vec<u32>, (u32, u32), (u32, u32)),
    TripleError(TripleErrorReason),
}

impl std::fmt::Display for BCHError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidInput(_) => write!(f, "An invalid digit was found in the given string"),
            SingleError(corrected, pos, mag) => write!(f, "An error of magnitude {} was located at position {}. The string has been corrected to {}", mag, pos, corrected.iter().map(|&d| std::char::from_digit(d, 10).unwrap()).collect::<String>()),
            DoubleError(corrected, (pos1, pos2), (mag1, mag2)) => write!(f, "Two errors of magnitudes {} and {} were found at positions {} and {}. The string has been corrected to {}", mag1, mag2, pos1, pos2, corrected.iter().map(|&d| std::char::from_digit(d, 10).unwrap()).collect::<String>()),
            TripleError(reason) => write!(f, "More than two errors were detected. The reason given is: '{}'", reason)
        }
    }
}

#[derive(Debug, Serialize)]
pub enum TripleErrorReason {
    DivisionError,
    ErrorAtPositionZero,
    NoValidRoots,
    ValueCorrectedToTen,
}

impl std::fmt::Display for TripleErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DivisionError => {
                write!(f, "Encountered an error while attempting modular division")
            }
            ErrorAtPositionZero => {
                write!(f, "An error was present at location 0")
            }
            NoValidRoots => {
                write!(f, "No valid roots existed for Q^2 - 4PR")
            }
            ValueCorrectedToTen => {
                write!(f, "A value was corrected to 10")
            }
        }
    }
}

pub fn encode_bch(string: &str) -> Result<String, BCHError> {
    match calculate_hamming_check_digits(string) {
        Ok(digits) => Ok(format!("{}{}", string, digits)),
        Err(error) => Err(BCHError::InvalidInput(error)),
    }
}

pub fn verify_bch_input(input: &str) -> Result<(), BCHError> {
    let mut ints = input
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
        .ok_or(InvalidInput(HammingError::InvalidDigit))?;

    let sv: Vec<Modular> = match super::hamming::generate_syndromes(input) {
        Ok(sv) => sv.iter().map(|&d| d.modulo(11)).collect::<Vec<Modular>>(),
        Err(err) => return Err(BCHError::InvalidInput(err)),
    };

    if sv == [0, 0, 0, 0] {
        return Ok(());
    }

    let (p, q, r) = {
        (
            sv[1].pow(2) - sv[0] * sv[2],
            sv[0] * sv[3] - sv[1] * sv[2],
            sv[2].pow(2) - sv[1] * sv[3],
        )
    };

    return match (p.v(), q.v(), r.v()) {
        (0, 0, 0) => {
            let position = sv[1].try_div(sv[0]).ok_or(TripleError(DivisionError))?;
            let magnitude = sv[0];
            if position == 0 {
                return Err(TripleError(ErrorAtPositionZero));
            }
            let correct_digit = (ints[position.value() as usize - 1] as i32).modulo(11) - magnitude;
            ints[position.v() as usize - 1] = correct_digit.v() as u32;

            Err(SingleError(ints, position.v() as u32, magnitude.v() as u32))
        }
        _ => {
            let root = (q.v().pow(2) - (4 * p.v() * r.v()))
                .modulo(11)
                .sqrt()
                .ok_or(TripleError(NoValidRoots))?;

            let pos1 = (root - q).try_div(p * 2.modulo(11)).ok_or(TripleError(DivisionError))?;
            let pos2 = (0.modulo(11) - q - root)
                .try_div(p * 2.modulo(11))
                .ok_or(TripleError(DivisionError))?;

            if pos1 == 0 || pos2 == 0 {
                return Err(TripleError(ErrorAtPositionZero));
            };

            let mag2 = (pos1 * sv[0] - sv[1])
                .try_div(pos1 - pos2)
                .ok_or(TripleError(DivisionError))?;

            let mag1 = sv[0] - mag2;

            ints[pos1.v() as usize - 1] = (ints[pos1.v() as usize - 1] + (11.modulo(11) - mag1).v() as u32)
                .modulo(11)
                .v() as u32;
            ints[pos2.v() as usize - 1] = (ints[pos2.v() as usize - 1] + (11.modulo(11) - mag2).v() as u32)
                .modulo(11)
                .v() as u32;

            if ints.iter().any(|&elem| elem > 9) {
                return Err(TripleError(ValueCorrectedToTen));
            }

            Err(DoubleError(
                ints,
                (pos1.v() as u32, pos2.v() as u32),
                (mag1.v() as u32, mag2.v() as u32),
            ))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn validate_correct_inputs() {
        verify_bch_input("3745195876").unwrap();
    }
    #[test]
    pub fn correct_single_error() {
        let y = verify_bch_input("3945195876").unwrap_err();
        match y {
            SingleError(output, pos, mag) => {
                assert_eq!((output, pos, mag), (vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], 2, 2))
            }
            err => panic!("Wrong error type returned. Error was: {:?}", err),
        }
    }

    #[test]
    pub fn correct_double_errors() {
        let inputs = vec![
            ("3715195076", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (8, 3), (3, 8)),
            ("0743195876", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (4, 1), (9, 8)),
            ("3745195840", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (10, 9), (5, 8)),
            ("8745105876", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (6, 1), (2, 5)),
            ("3745102876", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (6, 7), (2, 8)),
            ("1145195876", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (1, 2), (9, 5)),
            ("3745191976", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (8, 7), (1, 7)),
            ("3745190872", vec![3, 7, 4, 5, 1, 9, 5, 8, 7, 6], (7, 10), (6, 7)),
        ];

        inputs.iter().for_each(|row| {
            let result = verify_bch_input(row.0).unwrap_err();
            match result {
                DoubleError(output, pos, mag) => {
                    assert_eq!(row.1, output);
                    assert_eq!(row.2, (pos));
                    assert_eq!(row.3, (mag));
                }
                _ => panic!("Wrong error type returned"),
            }
        });
    }

    #[test]
    pub fn warn_on_triple_error() {
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
            let result = verify_bch_input(row).unwrap_err();
            match result {
                TripleError(_) => {}
                _ => panic!("Wrong error type returned - {:?}", result),
            }
        });
    }
}
