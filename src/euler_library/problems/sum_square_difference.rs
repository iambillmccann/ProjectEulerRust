pub fn compute() -> String {
    (::euler_library::math_library::square(::euler_library::math_library::sum_natural(100)) - ::euler_library::math_library::sum_natural_squares(100)).to_string()
}
