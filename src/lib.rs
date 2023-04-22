//! # Toy RSA Library
//! ### Stuart Rimel, Rust, Spring 2023
//!
//! This library provides RSA key generation, encryption and decryption.
//!
//! This crate is an exercise and should not be used for anything that
//! needs to be kept secure in real-life.

use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Returns the least common multple of `p-1` and `q-1`.
pub fn carmichael_totient(p: u64, q: u64) -> u64 {
    lcm(p - 1, q - 1)
}

/// Generates a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    let (mut p, mut q);
    loop {
        p = rsa_prime();
        q = rsa_prime();
        let totient = carmichael_totient(p as u64, q as u64);
        if EXP < totient && gcd(EXP, totient) == 1 {
            break;
        }
    }
    (p, q)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

/// Decrypt the ciphertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d = modinverse(EXP, carmichael_totient(key.0 as u64, key.1 as u64));
    modexp(msg, d, key.0 as u64 * key.1 as u64) as u32
}
