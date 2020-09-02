use num_bigint::{BigUint, ToBigUint, BigInt, ToBigInt};
use num_traits::{Zero, One};
// use std::mem::replace;

fn elliptic_curve(x: BigUint, y: BigUint, p: BigUint) -> bool
{
    // ! NOTDONE
    let A = BigUint::parse_bytes(b"2060348749492490789823288813930625381761", 10).unwrap();
    let B = BigUint::parse_bytes(b"1991637506671384833171818673149062805973", 10).unwrap();

    println!("{:?}", BigUint::modpow(&y, &ToBigUint::to_biguint(&2).unwrap(), &p));
    println!("{:?}", (BigUint::modpow(&x, &ToBigUint::to_biguint(&3).unwrap(), &ToBigUint::to_biguint(&1).unwrap()) + (A % p.clone()) * x + (B % p.clone())) % p.clone());
    return false;
    // return BigUint::modpow(&y, &ToBigUint::to_biguint(&2).unwrap(), &p) ==
    // (BigUint::modpow(&x, &ToBigUint::to_biguint(&3).unwrap(), &ToBigUint::to_biguint(&1).unwrap()) + (A % p.clone()) * x + (B % p.clone())) % p.clone();
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

fn main() {
    let p = ToBigUint::to_biguint(&21503).unwrap();
    println!("Hello, world! {:?}", elliptic_curve(ToBigUint::to_biguint(&3).unwrap(), ToBigUint::to_biguint(&3).unwrap(), p));
}
