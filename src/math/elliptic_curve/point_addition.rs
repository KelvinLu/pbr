//! Elliptic curve point addition.

use std::ops;

use crate::math::elliptic_curve::point::EllipticCurvePoint;
use crate::math::elliptic_curve::point::Point;

/// Point addition.
impl ops::Add<EllipticCurvePoint> for EllipticCurvePoint {
    type Output = EllipticCurvePoint;

    /// Adds another elliptic curve point.
    fn add(self, rhs: EllipticCurvePoint) -> Self::Output {
        match (self, rhs) {
            (Self::PointAtInfinity, Self::PointAtInfinity) => Self::PointAtInfinity,
            (Self::PointOnCurve(point), Self::PointAtInfinity) => Self::PointOnCurve(point),
            (Self::PointAtInfinity, Self::PointOnCurve(point)) => Self::PointOnCurve(point),
            (Self::PointOnCurve(point_1), Self::PointOnCurve(point_2)) => {
                point_1 + point_2
            },
        }
    }
}

/// Point subtraction.
impl ops::Sub<EllipticCurvePoint> for EllipticCurvePoint {
    type Output = EllipticCurvePoint;

    /// Subtracts another elliptic curve point.
    fn sub(self, rhs: EllipticCurvePoint) -> Self::Output {
        match (self, rhs) {
            (Self::PointAtInfinity, Self::PointAtInfinity) => Self::PointAtInfinity,
            (Self::PointOnCurve(point), Self::PointAtInfinity) => Self::PointOnCurve(point),
            (Self::PointAtInfinity, Self::PointOnCurve(point)) => Self::PointOnCurve(point),
            (Self::PointOnCurve(point_1), Self::PointOnCurve(point_2)) => {
                point_1 - point_2
            },
        }
    }
}

/// Point negation.
impl ops::Neg for EllipticCurvePoint{
    type Output = EllipticCurvePoint;

    /// Returns the negative of the point.
    fn neg(self) -> Self::Output {
        match self {
            Self::PointAtInfinity => Self::PointAtInfinity,
            Self::PointOnCurve(point) => -point,
        }
    }
}

/// Point addition.
impl ops::Add<Point> for Point {
    type Output = EllipticCurvePoint;

    /// Adds another elliptic curve point.
    fn add(self, rhs: Point) -> Self::Output {
        self.assert_same_curve(&rhs);

        if self.x == rhs.x {
            if self.y == rhs.y {
                if self.y.is_zero() {
                    // Same point at (x, 0) -- vertical tangent/slope
                    EllipticCurvePoint::PointAtInfinity
                } else {
                    // Same point at (x, y) -- reflected tangent intercept point
                    let s = ((3 * (self.x * self.x)) + self.a) / (2 * self.y);
                    let x = (s * s) - (2 * self.x);
                    let y = (s * (self.x - x)) - self.y;

                    EllipticCurvePoint::PointOnCurve(Self::new((x, y), (self.a, self.b)).unwrap())
                }
            } else {
              // Two reflected points at the same x coordinate (x, y), (x, -y) -- vertical slope
              EllipticCurvePoint::PointAtInfinity
            }
        } else {
            // Two different points -- reflected slope intercept point
            let s = (self.y - rhs.y) / (self.x - rhs.x);
            let x = (s * s) - self.x - rhs.x;
            let y = (s * (rhs.x - x)) - rhs.y;

            EllipticCurvePoint::PointOnCurve(Self::new((x, y), (self.a, self.b)).unwrap())
        }
    }
}

/// Point subtraction.
impl ops::Sub<Point> for Point {
    type Output = EllipticCurvePoint;

    /// Subtracts another elliptic curve point.
    fn sub(self, rhs: Point) -> Self::Output {
        self.assert_same_curve(&rhs);

        if self.x == rhs.x {
            if self.y == rhs.y {
                if self.y.is_zero() {
                    // Same point at (x, 0) -- vertical tangent/slope
                    EllipticCurvePoint::PointAtInfinity
                } else {
                    // Same point at (x, y) -- tangent intercept point
                    let s = ((3 * (self.x * self.x)) + self.a) / (2 * self.y);
                    let x = (s * s) - (2 * self.x);
                    let y = self.y - (s * (self.x - x));

                    EllipticCurvePoint::PointOnCurve(Self::new((x, y), (self.a, self.b)).unwrap())
                }
            } else {
              // Two reflected points at the same x coordinate (x, y), (x, -y) -- vertical slope
              EllipticCurvePoint::PointAtInfinity
            }
        } else {
            // Two different points -- slope intercept point
            let s = (self.y - rhs.y) / (self.x - rhs.x);
            let x = (s * s) - self.x - rhs.x;
            let y = rhs.y - (s * (rhs.x - x));

            EllipticCurvePoint::PointOnCurve(Self::new((x, y), (self.a, self.b)).unwrap())
        }
    }
}

/// Point negation.
impl ops::Neg for Point {
    type Output = EllipticCurvePoint;

    /// Returns the negative of the point.
    fn neg(self) -> Self::Output {
        EllipticCurvePoint::PointOnCurve(Self::new((self.x, -(self.y)), (self.a, self.b)).unwrap())
    }
}
