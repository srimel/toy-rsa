use toy_rsa_lib::*;

pub const EXP: u64 = 65_537;

fn carmichael_totient(p : u64, q : u64) -> u64 {
    toy_rsa_lib::lcm(p-1, q-1)
}

pub fn genkey() -> (u32, u32) {
    let (mut p, mut q) = (0, 0);
    loop {
        p = rsa_prime();
        q = rsa_prime();
        let lambda = carmichael_totient(p as u64, q as u64);
        if EXP < lambda && gcd(EXP, lambda) == 1 {
            break;
        }
    }
    (p,q)
}

pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

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
    fn testing_decrypt() {
        let expected_msg: u32 = 42069;
        let private_key: (u32, u32) = genkey();
        let public_key: u64 = private_key.0 as u64 * private_key.1 as u64;
        let encrypted_msg = encrypt(public_key, expected_msg);
        let decrypted_msg = decrypt(private_key, encrypted_msg);
        assert_eq!(expected_msg, decrypted_msg);
    }


}
