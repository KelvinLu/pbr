//! Elliptic curve points.

use crate::math::finite_field_element::FiniteFieldElement;
use crate::math::number::Number;

/// A point on an elliptic curve.
///
/// Includes representation for the point at infinity.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum EllipticCurvePoint {
    /// A point on an elliptic curve.
    PointOnCurve(Point),

    /// The point at infinity.
    PointAtInfinity,
}

impl std::fmt::Display for EllipticCurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::PointOnCurve(point) => write!(f, "{}", point),
            Self::PointAtInfinity => write!(f, "point at infinity"),
        }
    }
}

/// Error when a given point is not on the elliptic curve.
#[derive(Debug)]
pub struct PointNotOnCurveError;

impl EllipticCurvePoint {
    fn new(coordinates: (Number, Number), parameters: (Number, Number)) -> Result<Self, PointNotOnCurveError> {
        let (x, y) = coordinates;
        let (a, b) = parameters;

        match Point::new((x, y), (a, b)) {
            Some(point) => {
                Ok(Self::PointOnCurve(point))
            },
            None => Err(PointNotOnCurveError),
        }
    }
}

impl TryFrom<((FiniteFieldElement, FiniteFieldElement), (FiniteFieldElement, FiniteFieldElement))> for EllipticCurvePoint {
    type Error = PointNotOnCurveError;

    /// Create an elliptic curve point based on a finite field.
    fn try_from(coordinates_and_parameters: ((FiniteFieldElement, FiniteFieldElement), (FiniteFieldElement, FiniteFieldElement))) -> Result<Self, Self::Error> {
        let ((x, y), (a, b)) = coordinates_and_parameters;

        Self::new(
            (Number::FiniteFieldElement(x), Number::FiniteFieldElement(y)),
            (Number::FiniteFieldElement(a), Number::FiniteFieldElement(b))
        )
    }
}

impl TryFrom<((f64, f64), (f64, f64))> for EllipticCurvePoint {
    type Error = PointNotOnCurveError;

    /// Create an elliptic curve point based on the field of real numbers.
    fn try_from(coordinates_and_parameters: ((f64, f64), (f64, f64))) -> Result<Self, Self::Error> {
        let ((x, y), (a, b)) = coordinates_and_parameters;

        Self::new(
            (Number::Float(x), Number::Float(y)),
            (Number::Float(a), Number::Float(b))
        )
    }
}

impl TryFrom<((i32, i32), (i32, i32))> for EllipticCurvePoint {
    type Error = PointNotOnCurveError;

    /// Create an elliptic curve point based on the field of real numbers.
    ///
    /// Parameters may be expressed as integers, which are converted into float point numbers.
    fn try_from(coordinates_and_parameters: ((i32, i32), (i32, i32))) -> Result<Self, Self::Error> {
        let ((x, y), (a, b)) = coordinates_and_parameters;

        Self::new(
            (Number::Float(x.into()), Number::Float(y.into())),
            (Number::Float(a.into()), Number::Float(b.into()))
        )
    }
}

/// A point on an elliptic curve.
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Point {
    /// `x` coordinate.
    pub x: Number,

    /// `y` coordinate.
    pub y: Number,

    /// Parameter `a` of the elliptic curve.
    pub a: Number,

    /// Parameter `b` of the elliptic curve.
    pub b: Number,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}) on curve y^2=x^3+({})x+({})", self.x, self.y, self.a, self.b)
    }
}

impl Point {
    /// Create an elliptic curve point, expressed as a set of point coordinates alongside the
    /// elliptic curve parameters `a` and `b`.
    ///
    /// ```
    /// y^2 = x^3 + a*x + b
    /// ```
    ///
    /// If the point is not on the elliptic curve, nothing is returned instead.
    pub fn new(coordinates: (Number, Number), parameters: (Number, Number)) -> Option<Self> {
        let (x, y) = coordinates;
        let (a, b) = parameters;

        match (Self { x: x, y: y, a: a, b: b }) {
            point if point.is_on_curve() => Some(point),
            _ => None,
        }
    }

    fn is_on_curve(&self) -> bool {
        let y_squared = self.y * self.y;
        let x_cubed = self.x * self.x * self.x;

        y_squared == x_cubed + (self.a * self.x) + self.b
    }
}

impl Point {
    /// Asserts that another elliptic curve point is on the same curve.
    pub fn assert_same_curve(&self, rhs: &Point) {
        assert_eq!(self.a, rhs.a);
        assert_eq!(self.b, rhs.b);
    }
}
