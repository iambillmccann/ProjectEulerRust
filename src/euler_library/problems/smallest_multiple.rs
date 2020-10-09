/// <summary>
/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
/// </summary>
/// <returns></returns>
pub fn compute() -> String {

    fn all_are_factors(n: u64, trial: u64) -> bool {
        if n == 1 { true }
        else if !(::euler_library::math_library::is_multiple(trial, n)) { false }
        else { all_are_factors(n - 1, trial) }
    }

    fn find_solution(n: u64, trial: u64) -> u64 {
        let mut result = trial;
        while !all_are_factors(n, result) { result = result + trial }
        result
    }

    let n: u64 = 20;
    let primes = ::euler_library::math_library::get_prime(n);
    let mut gap = 1;
    for prime in primes { gap = gap * prime; }
    let result = find_solution(n, gap);
    result.to_string()
}
