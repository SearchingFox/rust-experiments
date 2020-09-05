#![feature(try_trait)]
use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use std::time::Instant;

const C1: &[u8; 40] = b"1860348749492490789823288813930625381760";
const C2: &[u8; 40] = b"2001637506671384833171818673149062805974";

fn in_elliptic_curve(x: BigInt, y: BigInt, p: BigInt) -> bool {
    let a = BigInt::parse_bytes(C1, 10).unwrap();
    let b = BigInt::parse_bytes(C2, 10).unwrap();

    BigInt::modpow(&y, &2.to_bigint().unwrap(), &p)
        == (&x * &x * &x + (a % &p) * &x + (b % &p)) % &p
}

fn extended_euclidean_algorithm(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut s, mut old_s): (BigInt, BigInt) = (Zero::zero(), One::one());
    let (mut t, mut old_t): (BigInt, BigInt) = (One::one(), Zero::zero());
    let (mut r, mut old_r) = (b, a);

    while r != Zero::zero() {
        let quotient = &old_r / &r;
        let tmp = old_r.clone();
        old_r = r.clone();
        r = tmp - &quotient * r;
        let tmp = old_s.clone();
        old_s = s.clone();
        s = tmp - &quotient * s;
        let tmp = old_t.clone();
        old_t = t.clone();
        t = tmp - quotient * t;
    }

    (old_r, old_s, old_t)
}

fn inverse_of(n: BigInt, p: BigInt) -> BigInt {
    let (gcd, x, y) = extended_euclidean_algorithm(n.clone(), p.clone());
    assert!(
        (&n * &x + &p * y) % &p == gcd
    );

    if gcd != 1.to_bigint().unwrap() {
        panic!("{} has no multiplicative inverse modulo {}", n, p);
    } else {
        return x % p;
    }
}

fn add_points(
    (x1, x2): (BigInt, BigInt),
    (y1, y2): (BigInt, BigInt),
    p: BigInt
) -> (BigInt, BigInt) {
    if (&x1, &x2) == (&Zero::zero(), &Zero::zero()) {
        return (y1.to_bigint().unwrap(), y2.to_bigint().unwrap());
    } else if (&y1, &y2) == (&Zero::zero(), &Zero::zero()) {
        return (x1.to_bigint().unwrap(), x2.to_bigint().unwrap());
    } else if &x1 == &y1 && &x2 != &y2 {
        return (Zero::zero(), Zero::zero());
    }

    let a = BigInt::parse_bytes(C1, 10).unwrap();
    let s = if (&x1, &x2) == (&y1, &y2) {
        ((3.to_bigint().unwrap() * &x1 * &x1 + (a % &p))
            * inverse_of(2.to_bigint().unwrap() * &x2, p.clone())) % &p
    } else {
        ((&x2 - &y2) * inverse_of(&x1 - &y1, p.clone())) % &p
    };
    let x = (&s * &s - 2.to_bigint().unwrap() * &x1) % &p;
    let y = (&x2 + s * (&x - &x1)) % &p;

    (x, -y % p)
}

fn point_order(point: &(BigInt, BigInt), p: BigInt) -> i32 {
    let mut count = 1;
    let mut check = add_points(point.clone(), point.clone(), p.clone());
    while check != (Zero::zero(), Zero::zero()) {
        check = add_points(check, point.clone(), p.clone());
        count += 1;
    }

    count
}

fn main() -> Result<(), std::option::NoneError> {
    let p = 250; // 13
    let mut points: Vec<(BigInt, BigInt)> = Vec::new();
    let start = Instant::now();

    for i in 0..p {
        for j in 0..p {
            if in_elliptic_curve(i.to_bigint()?, j.to_bigint()?, p.to_bigint()?) {
                points.push((i.to_bigint()?, j.to_bigint()?));
            }
        }
    }

    let a = BigInt::parse_bytes(C1, 10).unwrap();
    let b = BigInt::parse_bytes(C2, 10).unwrap();
    println!("y^2 = x^3 + {} * x + {} (mod {})", a % p.to_bigint().unwrap(), b % p.to_bigint().unwrap(), p);

    println!("Curve order = {}", &points.len());
    // println!("{:?}", points);
    let point = &points[0];
    println!("Point order {:?}: {:?}", point, point_order(point, p.to_bigint().unwrap()));
    println!("Time elapsed is: {:?}", start.elapsed());
    Ok(())
}
