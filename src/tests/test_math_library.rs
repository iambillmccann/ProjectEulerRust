#[cfg(test)]
pub mod test_math_library {
    use super::super::super::euler_library::math_library;

    #[test]
    fn reversing_even_number_of_digits() {
        assert_eq!(math_library::reverse_digits(1234),4321);
    }

    #[test]
    fn reversing_odd_number_of_digits() {
        assert_eq!(math_library::reverse_digits(123456789),987654321);
    }

    #[test]
    fn pass_is_multiple() {
        assert_eq!(math_library::is_multiple(4, 2), true);
        assert_eq!(math_library::is_multiple(9, 3), true);
        assert_eq!(math_library::is_multiple(12, 6), true);
        assert_eq!(math_library::is_multiple(25, 5), true);
        assert_eq!(math_library::is_multiple(4648, 332), true);
        assert_eq!(math_library::is_multiple(2, 1), true);
    }

    #[test]
    fn fail_is_multiple() {
        assert_eq!(math_library::is_multiple(4, 3), false);
        assert_eq!(math_library::is_multiple(1, 0), false);
        assert_eq!(math_library::is_multiple(13, 3), false);
        assert_eq!(math_library::is_multiple(19, 18), false);
        assert_eq!(math_library::is_multiple(23, 2), false);
        assert_eq!(math_library::is_multiple(37, 66), false);
    }

    #[test]
    fn compute_square() {
        assert_eq!(math_library::square(5), 25);
        assert_eq!(math_library::square(0), 0);
    }

    #[test]
    fn compute_sum_natural() {
        assert_eq!(math_library::sum_natural(10), 55);
        assert_eq!(math_library::sum_natural(0), 0);
    }

    #[test]
    fn compute_sum_squares() {
        assert_eq!(math_library::sum_natural_squares(10), 385);
        assert_eq!(math_library::sum_natural_squares(0), 0);
    }

    #[test]
    fn compute_get_prime() {
        let actual: Vec<u64> = math_library::get_prime(20);
        let expected: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(expected, actual);

        assert_eq!(9, (math_library::get_prime(23)).len());

        let expected: u64 = 7919;
        let prime_numbers: Vec<u64> = math_library::get_prime(7920);
        let actual: u64 = prime_numbers[prime_numbers.len() - 1];
        assert_eq!(expected, actual);
    }

    #[test]
    fn compute_series_product() {
        let numbers: Vec<u64> = vec![2, 3, 4, 5];
        assert_eq!(120, math_library::series_product(numbers));
        let numbers: Vec<u64> = vec![0, 999, 33, 10];
        assert_eq!(0, math_library::series_product(numbers));
        let numbers: Vec<u64> = vec![9, 9, 8, 9];
        assert_eq!(5832, math_library::series_product(numbers));
    }

    #[test]
    fn compute_series_sum() {
        let numbers: Vec<u64> = vec![2, 3, 5, 7];
        assert_eq!(17, math_library::series_sum(numbers));
    }

}