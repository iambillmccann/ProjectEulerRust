/// <summary>
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
/// Find the sum of all the primes below two million.
/// </summary>
/// <returns></returns>
pub fn compute() -> String {
    let prime_numbers: Vec<u64> = ::euler_library::math_library::get_prime(2000000);
    (::euler_library::math_library::series_sum(prime_numbers)).to_string()
}
