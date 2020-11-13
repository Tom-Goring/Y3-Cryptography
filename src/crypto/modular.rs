use gcd::Gcd;
use num::Integer;
use std::fmt::{Display, Formatter};
use std::{fmt, ops};

// TODO: replace assertions with results?
// TODO: replace i32 with T

#[derive(Copy, Clone, Debug)]
pub struct Modular {
    value: i32,
    modulo: i32,
}

impl Modular {
    pub fn new(value: i32, modulo: i32) -> Self {
        assert!(modulo > 0);
        Self {
            value: value.rem_euclid(modulo),
            modulo,
        }
    }

    pub fn try_div(&self, rhs: Self) -> Result<Self, &str> {
        if self.modulo != rhs.modulo {
            return Err("Attempted division by a value of differing modulo");
        };
        if rhs.value.gcd(&self.modulo) != 1 {
            return Err("The RHS value did not satisfy the restraint of having a GCD of 1 with the LHS modulo.");
        };
        if rhs.value == 0 {
            return Err("Attempted divide by zero");
        }

        let mut acc = self.value;

        loop {
            if acc % rhs.value == 0 {
                break;
            } else {
                acc += self.modulo;
            }
        }

        Ok(Self::new(acc / rhs.value, self.modulo))
    }

    pub fn sqrt(&self) -> Result<Self, ()> {
        for x in 1..self.modulo {
            if Modular::new(x, self.modulo).pow(2) == self.value {
                return Ok(Self {
                    value: x,
                    modulo: self.modulo,
                });
            }
        }
        return Err(());
    }

    pub fn pow(self, power: i32) -> Self {
        (1..power)
            .into_iter()
            .fold(Modular::new(self.value, self.modulo), |acc, _| acc * self)
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl ops::Add for Modular {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.modulo, rhs.modulo);
        let new_value = (self.value + rhs.value).rem_euclid(self.modulo);
        Self::new(new_value, self.modulo)
    }
}

impl ops::Sub for Modular {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.modulo, rhs.modulo);
        let new_value = (self.value - rhs.value).rem_euclid(self.modulo);
        Self::new(new_value, self.modulo)
    }
}

impl ops::Mul for Modular {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.modulo, rhs.modulo);
        let new_value = (self.value * rhs.value).rem_euclid(self.modulo);
        Self::new(new_value, self.modulo)
    }
}

impl ops::Div for Modular {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.modulo, rhs.modulo);
        assert_eq!((rhs.value as u32).gcd_euclid(self.modulo as u32), 1);
        assert_ne!(rhs.value, 0);

        let mut acc = self.value;

        loop {
            if acc % rhs.value == 0 {
                break;
            } else {
                acc += self.modulo;
            }
        }

        Self::new(acc / rhs.value, self.modulo)
    }
}

impl PartialEq for Modular {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

impl PartialEq<i32> for Modular {
    fn eq(&self, other: &i32) -> bool {
        self.value == *other
    }

    fn ne(&self, other: &i32) -> bool {
        self.value != *other
    }
}

impl Eq for Modular {}

impl Display for Modular {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} mod {}", self.value, self.modulo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn addition() {
        let x = Modular::new(1, 11);
        let y = Modular::new(10, 11);
        let z = x + y;
        assert_eq!(z.value, 0);
    }

    #[test]
    pub fn subtraction() {
        let x = Modular::new(1, 11);
        let y = Modular::new(10, 11);
        let z = x - y;
        assert_eq!(z.value, 2);
    }

    #[test]
    pub fn multiplication() {
        let x = Modular::new(4, 11);
        let y = Modular::new(7, 11);
        let z = x * y;
        assert_eq!(z.value, 6);
        assert_eq!(z.modulo, 11);
    }

    #[test]
    pub fn division() {
        let x = Modular::new(-6, 11);
        let y = Modular::new(9, 11);
        let z = x / y;
        assert_eq!(z.value, 3);

        let x = Modular::new(-8, 11);
        let y = Modular::new(9, 11);
        let z = x / y;
        assert_eq!(z.value, 4);
    }

    #[test]
    pub fn pow() {
        let x = Modular::new(5, 11);
        assert_eq!(x.pow(2).value, 3);
        assert_eq!(x.pow(47).value, 3);
        assert_eq!(x.pow(34).value, 9);
        assert_eq!(x.pow(35).value, 1);
    }
}
