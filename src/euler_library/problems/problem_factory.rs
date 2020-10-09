// The problem_factory module is a class implementing a classic factory pattern
// to instantiate problem solutions
    
// get_solution is the factory method for creating a solution to each problem
pub fn get_solution(problem_number: i32) -> String {
    match problem_number {
        1 => super::multiple_3or5::compute(),
        2 => super::even_fibonacci::compute(),
        3 => super::largest_prime_factor::compute(),
        4 => super::largest_palindrome_product::compute(),
        5 => super::smallest_multiple::compute(),
        6 => super::sum_square_difference::compute(),
        7 => super::prime10001::compute(),
        8 => super::largest_product_series::compute(),
        9 => super::pythagorean_triplet::compute(),
        10 => super::summation_primes::compute(),
        11 => super::largest_product_grid::compute(),
        12 => super::highly_divisible_triangle::compute(),
        13 => super::large_sum::compute(),
        _ => "Problem ".to_owned() + &problem_number.to_string() + " is not solved.",
    }
}
