/// <summary>
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600,851,475,143?
/// </summary>
/// <returns></returns>
pub fn compute() -> String {
    let value = 600851475143;
    let factors: Vec<u64> = ::euler_library::math_library::get_factors_with(value, ::euler_library::math_library::get_prime(10000));
    let result = factors[factors.len() - 1]; 
    result.to_string()
}
