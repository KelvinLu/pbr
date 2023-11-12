//! Numbers.

use std::ops;

use crate::math::finite_field_element::FiniteFieldElement;
use crate::util::number::U256;

/// Represents a number.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Number {
    /// Represents a finite field element.
    FiniteFieldElement(FiniteFieldElement),

    /// Represents a floating point number.
    Float(f64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FiniteFieldElement(element) => write!(f, "{}", element),
            Self::Float(number) => write!(f, "{}", number),
        }
    }
}

/// Zero element.
impl Number {
    pub fn is_zero(&self) -> bool {
        match self {
            Self::FiniteFieldElement(element) => element.value == U256::ZERO,
            Self::Float(number) => *number == 0.0,
        }
    }
}

/// Addition.
impl ops::Add<Self> for Number {
    type Output = Self;

    /// Add another number.
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::FiniteFieldElement(lhs), Self::FiniteFieldElement(rhs)) => Self::FiniteFieldElement(lhs + rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs + rhs),
            _ => panic!("incompatible types"),
        }
    }
}

/// Subtraction.
impl ops::Sub<Self> for Number {
    type Output = Self;

    /// Subtract another number.
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::FiniteFieldElement(lhs), Self::FiniteFieldElement(rhs)) => Self::FiniteFieldElement(lhs - rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs - rhs),
            _ => panic!("incompatible types"),
        }
    }
}

/// Negation.
impl ops::Neg for Number {
    type Output = Self;

    /// Returns the additive inverse.
    fn neg(self) -> Self::Output {
        match self {
            Self::FiniteFieldElement(element) => Self::FiniteFieldElement(-element),
            Self::Float(number) => Self::Float(-number),
        }
    }
}

/// Multiplication.
impl ops::Mul<Self> for Number {
    type Output = Self;

    /// Multiply by another number.
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::FiniteFieldElement(lhs), Self::FiniteFieldElement(rhs)) => Self::FiniteFieldElement(lhs * rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs * rhs),
            _ => panic!("incompatible types"),
        }
    }
}

/// Division.
impl ops::Div<Self> for Number {
    type Output = Self;

    /// Divide by another number.
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::FiniteFieldElement(lhs), Self::FiniteFieldElement(rhs)) => Self::FiniteFieldElement(lhs / rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs / rhs),
            _ => panic!("incompatible types"),
        }
    }
}

/// Multiplication.
impl ops::Mul<Number> for u32 {
    type Output = Number;

    /// Multiply by another number.
    fn mul(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::FiniteFieldElement(element) => Number::FiniteFieldElement(self * element),
            Number::Float(number) => Number::Float(f64::from(self) * number),
        }
    }
}

/// Multiplication.
impl ops::Mul<Number> for i32 {
    type Output = Number;

    /// Multiply by another number.
    fn mul(self, rhs: Number) -> Self::Output {
        match rhs {
            Number::FiniteFieldElement(element) => Number::FiniteFieldElement(self * element),
            Number::Float(number) => Number::Float(f64::from(self) * number),
        }
    }
}
