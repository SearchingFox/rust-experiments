use num_bigint::{BigUint, ToBigUint, BigInt, ToBigInt};
use num_traits::{Zero, One};
use std::time::Instant;
// use std::mem::replace;

fn in_elliptic_curve(x: BigUint, y: BigUint, p: BigUint) -> bool {
    let a = BigUint::parse_bytes(b"2060348749492490789823288813930625381761", 10).unwrap();
    let b = BigUint::parse_bytes(b"1991637506671384833171818673149062805973", 10).unwrap();
    return BigUint::modpow(&y, &2.to_biguint().unwrap(), &p) == (&x * &x * &x + (a % &p) * &x + (b % &p)) % &p
}

fn extended_euclidean_algorithm(a: BigUint, b: BigUint) -> (BigInt, BigInt, BigInt)
{
    let (mut s, mut old_s): (BigInt, BigInt) = (Zero::zero(), One::one());
    let (mut t, mut old_t): (BigInt, BigInt) = (One::one(), Zero::zero());
    let (mut r, mut old_r) = (b.to_bigint().unwrap(), a.to_bigint().unwrap());
    let mut j;
    while r != Zero::zero() {
        // TODO: wtf is this
        let quotient = old_r.clone() / r.clone();
        j = old_r.clone();
        old_r = r.clone();
        r = j.clone() - quotient.clone() * r;
        j = old_s.clone();
        old_s = s.clone();
        s = j.clone() - quotient.clone() * s;
        j = old_t.clone();
        old_t = t.clone();
        t = j.clone() - quotient * t;
    }

    return (old_r, old_s, old_t);
}

fn inverse_of(n: BigUint, p: BigUint) -> BigInt
{
    let (gcd, x, y) = extended_euclidean_algorithm(n.clone(), p.clone());
    assert!((n.to_bigint().unwrap() * &x + p.to_bigint().unwrap() * y) % p.to_bigint().unwrap() == gcd);

    if gcd != 1.to_bigint().unwrap() {
        panic!("{} has no multiplicative inverse modulo {}", n, p);
    } else {
        return x % p.to_bigint().unwrap()
    }
}

fn add_points((x1, x2): (BigInt, BigInt), (y1, y2): (BigInt, BigInt), p: BigInt) -> (BigInt, BigInt)
{
    let a = BigInt::parse_bytes(b"2060348749492490789823288813930625381761", 10).unwrap();
    if (&x1, &x2) == (&Zero::zero(), &Zero::zero()) {
        return (y1.to_bigint().unwrap(), y2.to_bigint().unwrap())
    } else if (&y1, &y2) == (&Zero::zero(), &Zero::zero()) {
        return (x1.to_bigint().unwrap(), x2.to_bigint().unwrap())
    } else if &x1 == &y1 && &x2 != &y2 {
        return (Zero::zero(), Zero::zero())
    }

    let s = if (&x1, &x2) == (&y1, &y2) {
        ((3.to_bigint().unwrap() * &x1 * &x1 + (a % &p)) * inverse_of((2.to_bigint().unwrap() * &x2).to_biguint().unwrap(), p.to_biguint().unwrap())) % &p
    } else {
         ((&x2 - &y2) * inverse_of((&x1 - &y1).to_biguint().unwrap(), p.to_biguint().unwrap())) % &p
    };
    let x = (&s * &s - 2.to_bigint().unwrap() * &x1) % &p;
    let y = (&x2 + s * (&x - &x1)) % &p;
    return (x, -y % p)
}

fn order_point(point: &(BigInt, BigInt), p: BigInt) -> i64
{
    let mut i = 1;
    let mut check = add_points(point.clone(), point.clone(), p.clone());
    while check != (Zero::zero(), Zero::zero()) {
        check = add_points(check, point.clone(), p.clone());
        i += 1;
    }
    return i
}

fn main() {
    let p = 250; // 13
    let mut points: Vec<(BigInt, BigInt)> = Vec::new();
    let start = Instant::now();

    for i in 0..p {
        for j in 0..p {
            if in_elliptic_curve(i.to_biguint().unwrap()
                               , j.to_biguint().unwrap()
                               , p.to_biguint().unwrap()) {
                points.push((i.to_bigint().unwrap()
                           , j.to_bigint().unwrap()));
            }
        }
    }
    
    println!("Curve order = {}", &points.len());
    let point = &points[10];
    println!("Point order {:?}: {:?}", point, order_point(point, p.to_bigint().unwrap()));
    println!("Time elapsed is: {:?}", start.elapsed());
}
