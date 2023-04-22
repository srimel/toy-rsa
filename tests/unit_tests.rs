use toy_rsa::*;

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
