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

}