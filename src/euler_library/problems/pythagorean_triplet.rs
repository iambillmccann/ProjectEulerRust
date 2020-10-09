/// <summary>
/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
/// </summary>
/// <returns></returns>
pub fn compute() -> String {
        
    for a in 1 .. 334 {
        let mut done: bool = false;
        let mut b: u64 = a + 1;
        while !done {
            let c: u64 = 1000 - (a + b);
            if ::euler_library::math_library::square(a) + ::euler_library::math_library::square(b) == ::euler_library::math_library::square(c) {
                return (a * b * c).to_string();
            }
            b += 1;
            if b >= c { done = true }
        }
    }

    "No result found".to_string()
}
