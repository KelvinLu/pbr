use programming_bitcoin_in_rust::*;
use crate::util::println_exercise::*;

use crypto::secp256k1;
use math::elliptic_curve::point::EllipticCurvePoint;
use math::finite_field_element::FiniteFieldElement;
use math::number::Number;
use util::number::U256;
use util::number::uint;

pub fn exercise() {
    prompt("Generator points, secp256k1");

    section("Elliptic curve (y ^ 2) = (x ^ 3) + 7, over finite field of prime order 223");

    let a = FiniteFieldElement::from((0, 223_u32));
    let b = FiniteFieldElement::from((7, 223_u32));

    let x = FiniteFieldElement::from((15, 223_u32));
    let y = FiniteFieldElement::from((86, 223_u32));
    let point = EllipticCurvePoint::try_from(((x, y), (a, b))).unwrap();

    message(&format!("{}", point));

    let mut order = 0;
    let mut accumulator = EllipticCurvePoint::PointAtInfinity;

    loop {
        order += 1;
        accumulator = accumulator + point;

        message(&format!("{} ... {}", order, accumulator));

        if accumulator == EllipticCurvePoint::PointAtInfinity {
            break;
        }
    }

    show_display(&format!("The generator point {} produces a group with order {}", point, order));

    section("Elliptic curve (y ^ 2) = (x ^ 3) + 7, over finite field of prime order 'p'");

    let p = secp256k1::FINITE_FIELD_ORDER;

    show_display(&p);
    show_debug(&p);

    let a = FiniteFieldElement::new(secp256k1::ELLIPTIC_CURVE_A, p);
    let b = FiniteFieldElement::new(secp256k1::ELLIPTIC_CURVE_B, p);

    let x = FiniteFieldElement::new(secp256k1::GENERATOR_POINT_X, p);
    let y = FiniteFieldElement::new(secp256k1::GENERATOR_POINT_Y, p);
    let point = EllipticCurvePoint::try_from(((x, y), (a, b))).unwrap();

    section(&format!("secp256k1 generator point G = {}", point));

    let n = secp256k1::ELLIPTIC_CURVE_ORDER;

    message("secp256k1 elliptic curve has a finite point group of prime order 'n'");
    show_display(&n);
    show_debug(&n);

    section(&format!("{} * G =", n));
    show_display(&(n * point));

    uint! {
        let point = secp256k1::Secp256k1Point::new(
            0x04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574_U256,
            0x82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4_U256
        );

        let z = 0xbc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423_U256;

        let r = 0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6_U256;
        let s = 0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec_U256;
    }

    section("ECDSA: 'r' and 's' are signature values, produced for some data 'z' and verifiable against a public point");

    show_display(&point);
    message("z =");
    show_debug(&z);
    message("r =");
    show_debug(&r);
    message("s =");
    show_debug(&s);

    section("Signature verification");

    let s_inv = s.pow_mod(secp256k1::ELLIPTIC_CURVE_ORDER - U256::from(2), secp256k1::ELLIPTIC_CURVE_ORDER);
    let u = z.mul_mod(s_inv, secp256k1::ELLIPTIC_CURVE_ORDER);
    let v = r.mul_mod(s_inv, secp256k1::ELLIPTIC_CURVE_ORDER);

    message("s ^ -1 mod n =");
    show_debug(&s_inv);
    message("u = z / s mod n =");
    show_debug(&u);
    message("v = r / s mod n =");
    show_debug(&v);

    let result = (u * secp256k1::Secp256k1Point::generator_point()) + (v * point);

    message("(u * G) + (v * <point>) =");
    show_display(&result);

    section("Result has an 'x' coordinate that should match 'r' if the signature is valid");

    let secp256k1::Secp256k1Point::Point(result) = result;
    let EllipticCurvePoint::PointOnCurve(result) = result else { panic!("the calculation should not result with point at infinity") };

    show_debug(&result.x);

    let Number::FiniteFieldElement(element) = result.x else { panic!("expected a finite field element") };

    show_display(&(element.value == r));
}
