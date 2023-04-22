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
fn carmichael_totient(p: u64, q: u64) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_carmichael_totient() {
        assert_eq!(1495, carmichael_totient(24, 66));
        assert_eq!(12, carmichael_totient(4, 5));
    }

    #[test]
    fn testing_genkey() {
        let (p, q) = genkey();
        assert!(is_prime(p));
        assert!(is_prime(q));
        assert_ne!(p, q);
    }

    /// Checks primality of `n` and returns a boolean result.
    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f32).sqrt() as u32) {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn testing_is_prime() {
        assert!(is_prime(503));
        assert!(is_prime(42) == false);
    }

    #[test]
    fn testing_encrypt() {
        assert_eq!(1546352421, encrypt(2734948301, 424242));
    }

    #[test]
    fn testing_decrypt_from_example() {
        let msg = 0x12345fu32;
        let private_key = (0xed23e6cdu32, 0xf050a04du32);
        let public_key = private_key.0 as u64 * private_key.1 as u64;
        assert_eq!(0xde9c5816141c8ba9, public_key);
        let encrypted_msg = encrypt(public_key, msg);
        assert_eq!(0x6418280e0c4d7675, encrypted_msg);
        let decrypted_msg = decrypt(private_key, encrypted_msg);
        assert_eq!(msg, decrypted_msg);
    }

    #[test]
    fn testing_decrypt_from_genkey() {
        let expected_msg: u32 = 42069;
        let private_key: (u32, u32) = genkey();
        let public_key: u64 = private_key.0 as u64 * private_key.1 as u64;
        let encrypted_msg = encrypt(public_key, expected_msg);
        let decrypted_msg = decrypt(private_key, encrypted_msg);
        assert_eq!(expected_msg, decrypted_msg);
    }
}
