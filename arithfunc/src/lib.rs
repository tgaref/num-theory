extern crate gmp;
extern crate factor;
use gmp::mpz::Mpz;
use factor::pollard;

pub fn totient(n: &Mpz) -> Mpz {
    if *n == Mpz::zero() {
        Mpz::zero()
    } else if *n == Mpz::one() {
        Mpz::one()
    } else {
        let pes = pollard::factor(&n);
        let one = Mpz::one();
        let mut tot = Mpz::one();
        
        for (p, e) in pes.into_iter() {
            tot *= p.pow(e-1) * (&p - &one);
        }
        tot
    }
}

pub fn sigma(n: &Mpz) -> Mpz {
    let pes = pollard::factor(n);
    let one = Mpz::one();
    let mut result = Mpz::one();
    for (p, e) in pes.into_iter() {
        result *= (p.pow(e + 1) - &one) / (p - &one);
    }
    result
}

