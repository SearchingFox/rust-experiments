// #![feature(try_trait)]
// use num_Int::{Int, ToInt};
// use num_traits::{One, zero()};
use ramp::int::Int;
use std::time::Instant;

const C1: &str = "1860348749492490789823288813930625381760";
const C2: &str = "2001637506671384833171818673149062805974";

fn in_elliptic_curve(x: Int, y: Int, p: Int) -> bool {
    let a = Int::from_str_radix(C1, 10).unwrap();
    let b = Int::from_str_radix(C2, 10).unwrap();

    Int::pow_mod(&y, &Int::from(2), &p)
        == (&x * &x * &x + (a % &p) * &x + (b % &p)) % &p
}

fn extended_euclidean_algorithm(a: Int, b: Int) -> (Int, Int, Int) {
    let (mut s, mut old_s): (Int, Int) = (Int::zero(), Int::one());
    let (mut t, mut old_t): (Int, Int) = (Int::one(), Int::zero());
    let (mut r, mut old_r) = (b, a);

    while r != Int::zero() {
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

fn inverse_of(n: Int, p: Int) -> Int {
    let (gcd, x, y) = extended_euclidean_algorithm(n.clone(), p.clone());
    assert!(
        (&n * &x + &p * y) % &p == gcd
    );

    if gcd != Int::one() {
        panic!("{} has no multiplicative inverse modulo {}", n, p);
    } else {
        x % p
    }
}

fn add_points(
    (x1, x2): (Int, Int),
    (y1, y2): (Int, Int),
    p: Int
) -> (Int, Int) {
    if (&x1, &x2) == (&Int::zero(), &Int::zero()) {
        return (y1, y2);
    } else if (&y1, &y2) == (&Int::zero(), &Int::zero()) {
        return (x1, x2);
    } else if x1 == y1 && x2 != y2 {
        return (Int::zero(), Int::zero());
    }

    let a = Int::from_str_radix(C1, 10).unwrap();
    let s = if (&x1, &x2) == (&y1, &y2) {
        ((Int::from(3) * &x1 * &x1 + (a % &p))
            * inverse_of(Int::from(2) * &x2, p.clone())) % &p
    } else {
        ((&x2 - &y2) * inverse_of(&x1 - &y1, p.clone())) % &p
    };
    let x = (&s * &s - Int::from(2) * &x1) % &p;
    let y = (&x2 + s * (&x - &x1)) % &p;

    (x, -y % p)
}

fn point_order(point: &(Int, Int), p: Int) -> i32 {
    let mut count = 1;
    let mut check = add_points(point.clone(), point.clone(), p.clone());
    while check != (Int::zero(), Int::zero()) {
        check = add_points(check, point.clone(), p.clone());
        count += 1;
    }

    count
}

fn main() {
    let p = 2501; // 3
    let mut points: Vec<(Int, Int)> = Vec::new();
    let start = Instant::now();

    for i in 0..p {
        for j in 0..p {
            if in_elliptic_curve(Int::from(i), Int::from(j), Int::from(p)) {
                points.push((Int::from(i), Int::from(j)));
            }
        }
    }

    let a = Int::from_str_radix(C1, 10).unwrap();
    let b = Int::from_str_radix(C2, 10).unwrap();
    println!("y^2 = x^3 + {} * x + {} (mod {})", a % Int::from(p), b % Int::from(p), p);

    println!("Curve order = {}", &points.len());
    // println!("{:?}", points);
    let point = &points[0];
    // println!("Point order {:?}: {:?}", point, point_order(point, Int::from(p)));
    println!("Time elapsed is: {:?}", start.elapsed());
}
