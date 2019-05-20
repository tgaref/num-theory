extern crate gmp;
extern crate factor;
extern crate arithfunc;
use gmp::mpz::Mpz;
use factor::pollard;
use std::vec::Vec;

pub fn crt(avec: &Vec<Mpz>, nvec: &Vec<Mpz>) -> Mpz {
    let len = nvec.len();
    if len != avec.len() {
        panic!("Number of RHS values != number of moduli");
    }
    let mut n = Mpz::one();
    for ni in nvec.iter() {
        n *= ni;
    }
    let mut s;
    let mut mi;
    let mut result = Mpz::zero();
    for i in 0 .. len {
        mi = &n / &nvec[i];
        s = match mi.invert(&nvec[i]) {
            Some(v) => v,
            None    => panic!("The moduli are not pairwise coprime!"),
        };
        result += (&avec[i] * &mi) * s;
    }
    result % n
}

pub fn solve_linear(a: &Mpz, b: &Mpz, n: &Mpz) -> Vec<Mpz> {
    let d = a.gcd(n);
    let one = Mpz::one();
    if b % &d != Mpz::zero() {
        return vec![]
    } else {
        let n_ = n / &d;
        let a_ = a / &d;
        let b_ = b / &d;
        let mut x = (b_ * a_.invert(n).unwrap()) % &n_;
        let mut result = Vec::new();
        result.push(x.clone());
        let mut i = Mpz::one();
        while i < d {
            x += &n_;
            result.push(x.clone());
            i += &one;
        }
        result
    }
}

pub fn order(a: &Mpz, n: &Mpz) -> Mpz {
    if *a == Mpz::zero() || *n == Mpz::zero() {
        panic!("No order exists!");
    } else {
        let one = Mpz::one();
        let m = arithfunc::totient(n);
        let pes = pollard::factor(&m);
        let mut b;
        let mut q;
        let mut result = Mpz::one();
        for (p, e) in pes.into_iter() {
            q = Mpz::one();
            b = a.powm(&(&m / &p.pow(e)) , n);
            while b != one {
                q *= &p;
                b = b.powm(&p, n);
            }
            result *= q;
        }
        result
    }
}                    
