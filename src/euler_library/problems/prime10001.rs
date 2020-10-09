///
/// 10,0001th Prime number
/// 
    pub fn compute() -> String {
        let prime_numbers = ::euler_library::math_library::get_prime(110000);
        let result: u64 = prime_numbers[10001 - 1];
        result.to_string()
    }
