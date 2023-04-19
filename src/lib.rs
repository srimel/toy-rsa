use toy_rsa_lib::*;

pub fn add(left: usize, right: usize) -> usize {
    let result = toy_rsa_lib::modexp(10, 9, 6);
    let result2 = toy_rsa_lib::gcd(23415, 24512314313);
    let result3 = toy_rsa_lib::lcm(2342, 34526);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
