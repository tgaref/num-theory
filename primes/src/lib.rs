extern crate gmp;
use gmp::mpz::{Mpz, ProbabPrimeResult};
use std::vec::Vec;

pub fn primes(upto: &Mpz) -> Vec<Mpz> {
    let two = Mpz::from(2);
    let mut pr_vec = vec![two.clone()];
    let mut n = Mpz::from(3);
    while n <= *upto {
        if n.probab_prime(6) != ProbabPrimeResult::NotPrime {
            pr_vec.push(n.clone());
        }
        n += &two;
    }
    pr_vec
}
