extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
// use num_traits::{Zero, One};
// use std::mem::replace;

fn elliptic_curve(x: BigUint, y: BigUint, p: BigUint) -> bool
{
    let A = BigUint::parse_bytes(b"2060348749492490789823288813930625381761", 10).unwrap();
    let B = BigUint::parse_bytes(b"1991637506671384833171818673149062805973", 10).unwrap();

    println!("{:?}", BigUint::modpow(&y, &ToBigUint::to_biguint(&2).unwrap(), &p));
    println!("{:?}", (BigUint::modpow(&x, &ToBigUint::to_biguint(&3).unwrap(), &ToBigUint::to_biguint(&1).unwrap()) + (A % p.clone()) * x + (B % p.clone())) % p.clone());
    return false;
    // return BigUint::modpow(&y, &ToBigUint::to_biguint(&2).unwrap(), &p) ==
    // (BigUint::modpow(&x, &ToBigUint::to_biguint(&3).unwrap(), &ToBigUint::to_biguint(&1).unwrap()) + (A % p.clone()) * x + (B % p.clone())) % p.clone();
}

fn extended_euclidean_algorithm(a: BigUint, b: BigUint) -> (BigUint, BigUint, BigUint)
{
    let (mut s, mut old_s) = (ToBigUint::to_biguint(&0).unwrap(), ToBigUint::to_biguint(&1).unwrap());
    let (mut t, mut old_t) = (ToBigUint::to_biguint(&1).unwrap(), ToBigUint::to_biguint(&0).unwrap());
    let (mut r, mut old_r) = (b, a);

    while r != ToBigUint::to_biguint(&0).unwrap() {
        let quotient = old_r / r;
        old_r = r;
        r = old_r - quotient * r;
        old_s = s;
        s = old_s - quotient * s;
        old_t = t;
        t = old_t - quotient * t;
    }

    return (old_r, old_s, old_t);
}

fn main() {
    let p = ToBigUint::to_biguint(&21503).unwrap();
    println!("Hello, world! {:?}", elliptic_curve(ToBigUint::to_biguint(&3).unwrap(), ToBigUint::to_biguint(&3).unwrap(), p));
}
