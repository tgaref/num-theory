extern crate gmp;
use gmp::mpz::{Mpz, ProbabPrimeResult};
use gmp::rand::RandState;
use std::vec::Vec;
use crate::common;

fn g(x: &Mpz, n: &Mpz) -> Mpz {
    (x * x + Mpz::one()).mod_floor(n)
}

fn split(n: &Mpz) -> (Mpz, Mpz) {
    let rand_bound = Mpz::from(100);
    let one = Mpz::one();
    let mut x = RandState::new().urandom(&rand_bound);
    let mut y = x.clone();
    let mut d = Mpz::one();
    while d == one {
        x = g(&x, n);
        y = g(&g(&y, n), n);
        if x == y {
            x = RandState::new().urandom(&rand_bound);
            y = x.clone();
        } else {
            d = Mpz::abs(&(&x - &y)).gcd(n);
        }
    }
    (n.div_floor(&d), d)
}

pub fn prime_factors(n: &Mpz) -> Vec<Mpz> {
    if *n == Mpz::one() {
        return Vec::new()
    }
    let mut ps = Vec::new();
    let mut ns = vec![n.clone()];
    let mut m;
    while !ns.is_empty() {
        m = ns.pop().unwrap();
        if m.probab_prime(6) != ProbabPrimeResult::NotPrime {
            ps.push(m);
        } else {
            let (n1, n2) = split(&m);
            ns.push(n1);
            ns.push(n2);
        }     
    }
    ps.sort();
    ps
}

fn compute_pairs(ps: Vec<Mpz>) -> Vec<(Mpz,u32)> {
    if ps.is_empty() {
        return vec![]
    }
    let mut prev = 0;
    let mut e = 1;
    let mut result = Vec::new();
    for i in 1 .. ps.len() {
        if ps[i] == ps[prev] {
            e += 1;
        } else {
            result.push((ps[prev].clone(), e));
            e = 1;
            prev = i;
        }
        
    }
    result.push((ps[prev].clone(), e));
    result
}
                           
pub fn factor(n: &mut Mpz) -> Vec<(Mpz,u32)> {
    if *n == Mpz::zero() || *n == Mpz::one() {
        return vec![]
    }
    let mut ps1 = common::remove_primes(n, &Mpz::from(100));
    let mut ps2 = compute_pairs(prime_factors(n));
    ps1.append(&mut ps2);
    ps1
}
    
