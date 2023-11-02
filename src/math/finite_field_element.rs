//! Finite field elements.

use std::ops;

use crate::util::number::U256;

/// Represents a finite field element.
///
/// Supports representations within 256 bits.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(Eq, PartialEq, Ord)]
pub struct FiniteFieldElement {
    /// Value of the element.
    pub value: U256,

    /// Order of the finite field.
    pub cardinality: U256,
}

impl FiniteFieldElement {
    /// Create a new finite field element.
    pub fn new(value: U256, cardinality: U256) -> Self {
        Self {
            value: if value < cardinality { value } else {
                panic!("Value {} does not belong to finite field with cardinality {}", value, cardinality)
            },
            cardinality: cardinality,
        }
    }

    /// Assert that another finite field element belongs to the same finite field.
    fn assert_same_field(&self, rhs: &FiniteFieldElement) {
        assert_eq!(self.cardinality, rhs.cardinality)
    }
}

impl From<(u32, u32)> for FiniteFieldElement {
    /// Create a finite field element from a pair of unsigned 32 bit integers (element value and
    /// finite field order).
    fn from(value_and_cardinality: (u32, u32)) -> Self {
        let (value, cardinality) = value_and_cardinality;

        Self::new(U256::from(value), U256::from(cardinality))
    }
}

impl From<(i32, u32)> for FiniteFieldElement {
    /// Create a finite field element from a pair of a 32 bit integer (element value) and an
    /// unsigned 32 bit integer (finite field order).
    ///
    /// If the element value is negative, it is converted to being a whole number by taking its
    /// modulus around the finite field order.
    fn from(value_and_cardinality: (i32, u32)) -> Self {
        let (value, cardinality) = value_and_cardinality;

        Self::new(
            match value {
                n if n < 0 => U256::from(cardinality) - U256::from(-n),
                n => U256::from(n),
            },
            U256::from(cardinality)
        )
    }
}

impl std::fmt::Display for FiniteFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}_F{}", self.value, self.cardinality)
    }
}

/// Provides ordering for finite field elements within the same finite field (characterized by some
/// order).
///
/// This has no meaning for those between different finite fields (nonisophormic).
impl PartialOrd for FiniteFieldElement {
    /// Compares ordering against another finite field element within the same finite field.
    fn partial_cmp(&self, other: &FiniteFieldElement) -> Option<std::cmp::Ordering> {
        if self.cardinality == other.cardinality {
            Some(self.value.cmp(&other.value))
        } else {
            None
        }
    }
}

/// Finite field addition (within the same finite field).
impl ops::Add<FiniteFieldElement> for FiniteFieldElement {
    type Output = FiniteFieldElement;

    /// Adds another finite field element.
    fn add(self, rhs: FiniteFieldElement) -> Self::Output {
        self.assert_same_field(&rhs);
        FiniteFieldElement::new(self.value.add_mod(rhs.value, self.cardinality), self.cardinality)
    }
}

/// Finite field subtraction (within the same finite field).
impl ops::Sub<FiniteFieldElement> for FiniteFieldElement {
    type Output = FiniteFieldElement;

    /// Subtracts another finite field element.
    fn sub(self, rhs: FiniteFieldElement) -> Self::Output {
        self + (-rhs)
    }
}

/// Finite field additive inverse.
impl ops::Neg for FiniteFieldElement {
    type Output = FiniteFieldElement;

    /// Returns the additive inverse.
    fn neg(self) -> Self::Output {
        FiniteFieldElement::new(self.cardinality - self.value, self.cardinality)
    }
}

/// Finite field multiplication (within the same finite field).
impl ops::Mul<FiniteFieldElement> for FiniteFieldElement {
    type Output = FiniteFieldElement;

    /// Multiplies another finite field element.
    fn mul(self, rhs: FiniteFieldElement) -> Self::Output {
        self.assert_same_field(&rhs);
        FiniteFieldElement::new(self.value.mul_mod(rhs.value, self.cardinality), self.cardinality)
    }
}

/// Finite field multiplication.
impl ops::Mul<FiniteFieldElement> for U256 {
    type Output = FiniteFieldElement;

    /// Multiplies a finite field element by a whole number, expressed in the same finite field.
    fn mul(self, rhs: FiniteFieldElement) -> Self::Output {
        FiniteFieldElement::new(rhs.value.mul_mod(self, rhs.cardinality), rhs.cardinality)
    }
}

/// Finite field multiplication.
impl ops::Mul<FiniteFieldElement> for u32 {
    type Output = FiniteFieldElement;

    /// Multiplies a finite field element by a whole number, expressed in the same finite field.
    fn mul(self, rhs: FiniteFieldElement) -> Self::Output {
        U256::from(self) * rhs
    }
}

/// Finite field multiplication.
impl ops::Mul<FiniteFieldElement> for i32 {
    type Output = FiniteFieldElement;

    /// Multiplies a finite field element by an integer, expressed in the same finite field.
    ///
    /// Negative integer values are handled by multiplying the finite field element by the
    /// integer's absolute value first, followed by taking the resulting additive inverse.
    fn mul(self, rhs: FiniteFieldElement) -> Self::Output {
        match self {
            n if n < 0 => -(U256::from(-n) * rhs),
            n => U256::from(n) * rhs,
        }
    }
}

/// Finite field multiplication (within the same finite field).
impl ops::Div<FiniteFieldElement> for FiniteFieldElement {
    type Output = FiniteFieldElement;

    /// Divides another finite field element.
    ///
    /// This is expressed by multiplication of the modular invese.
    fn div(self, rhs: FiniteFieldElement) -> Self::Output {
        self * rhs.modular_inverse().unwrap()
    }
}

/// Finite field exponentiation.
impl FiniteFieldElement {
    /// Raises the finite field element by the given power.
    pub fn pow(self, exponent: U256) -> Self {
        Self::new(
            self.value.pow_mod(exponent, self.cardinality),
            self.cardinality
        )
    }

    /// Raises the finite field element by the given power.
    pub fn pow_u32(self, exponent: u32) -> Self {
        self.pow(U256::from(exponent))
    }

    /// Raises the finite field element by the given power.
    pub fn pow_i32(self, exponent: i32) -> Self {
        match exponent {
            n if n < 0 => self.modular_inverse().expect("no modular inverse").pow(U256::from(-n)),
            n => self.pow(U256::from(n))
        }
    }

    /// Returns the modular inverse of the finite field element (if one exists).
    pub fn modular_inverse(self) -> Option<Self> {
        Some(Self::new(self.value.inv_mod(self.cardinality)?, self.cardinality))
    }
}
