/// <summary>
/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
/// Find the largest palindrome made from the product of two 3-digit numbers
/// </summary>
/// <returns></returns>
pub fn compute() -> String {
    let mut result: u64 = 0;
    for number1 in 100..1000 {
        for number2 in number1..1000 {
            let product: u64 = number1 * number2;
            let string_version = product.to_string();
            if &string_version[0..1] == &string_version[string_version.chars().count()-1..] {
                if ::euler_library::math_library::reverse_digits(product) == product && product > result {
                    result = product;
                }
            }
        }
    }
    result.to_string()
}
