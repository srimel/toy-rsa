# Toy RSA Library
### Stuart Rimel, Rust, Srping 2023

<br>

This library provides RSA key generation, encryption and decryption.

This crate is an exercise and should not be used for anything that needs to be kept 
secure in real-life.

<br>

### What I did:
This solution utilized the provided [toy_rsa_lib](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/index.html) library for various functions such as finding 
the [least common multiple](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/fn.lcm.html), [greatest common divisor](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/fn.gcd.html), [getting a random prime](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/fn.rsa_prime.html),
[modexp](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/fn.modexp.html) for finding the modular exponent, and [modinverse](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/fn.modinverse.html) for finding the inverse mod of
two numbers. 

I used the pseudo code provided in the homework combined with the provided functions 
from `toy_rsa_lib` to implement my solution. 

<br>

```
E = 65537

𝜆(p, q):
    return least common multiple of p - 1 and q - 1

encrypt(key, msg):
    return msgE mod key

decrypt(key = p ⋅ q, msg):
    d ← inverse of E mod 𝜆(p, q)
    return msgd mod (p ⋅ q)

genkey:
    repeat 
        p, q ← rsa primes (primes in range 231 .. 232-1)
    until E < 𝜆(p, q) and gcd(E, 𝜆(p, q)) = 1
    return p, q
```

<br>

### How it went:
It was fun learning about RSA encryption and implementing a toy version of it. I was expecting to 
fight with type conversions, but using `as` casts made it pretty simple to implement. 

<br>

### How I tested:
I created a `tests/unit_tests.rs` file to contain all the test code for this library. 
I have at least one unit test for each function, with `decript` having two test cases. One is testing against
the provided test case given in the assignment, the other is testing from a `genkey` call.