extern crate gmp;
use gmp::mpz::Mpz;
use std::vec::Vec;
use primes;

fn remove(n: &mut Mpz, p: &Mpz) -> u32 {
    let mut e = 0;
    let zero = Mpz::zero();
    while n.mod_floor(p) == zero {
        e += 1;
        *n = n.div_floor(p);
    }
    e
}

pub fn remove_primes(n: &mut Mpz, upto: &Mpz) -> Vec<(Mpz, u32)> {
    let ps = primes::primes(upto);
    let mut result = Vec::new();
    let mut e: u32;
    for p in ps.into_iter() {
        e = remove(n, &p);
        if e > 0 {
            result.push((p, e));
        }
    }
    result
}
        
