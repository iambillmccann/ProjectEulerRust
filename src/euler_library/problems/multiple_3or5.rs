/// <summary>
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.
/// </summary>
/// <returns>return the answer to the problem</returns>
pub fn compute() -> String {
    let range_size : u64 = 1000 - 1;
    let sum_3s = ::euler_library::math_library::arithmetic_progression(range_size / 3, 3, 3);
    let sum_5s = ::euler_library::math_library::arithmetic_progression(range_size / 5, 5, 5);
    let sum_15s = ::euler_library::math_library::arithmetic_progression(range_size / 15, 15, 15);
    let result = sum_3s + sum_5s - sum_15s;
    result.to_string()
}
