pub fn arithmetic_progression(number_of_terms: u64, first_term: u64, difference: u64) -> u64 {

    let last_term: u64 = first_term + difference * (number_of_terms - 1);
    let arithmetic_series: u64 = number_of_terms * (first_term + last_term) / 2;
    arithmetic_series
}

/// <summary>
/// This method returns a collection of divisors for a given number
/// </summary>
/// <param name="number">The number being factored</param>
/// <returns>vector of divisors</returns>
pub fn get_divisors(number: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    let fnumber: f64 = number as f64;
    let limit: u64 = fnumber.sqrt().ceil() as u64 + 1;

    for iterate in 1..limit {
        let divisor: u64 = iterate as u64;
        if is_multiple(number, divisor) {
            divisors.push(divisor);
            let quotient: u64 = number / divisor;
            if divisor != quotient && number != divisor { divisors.push(quotient) }
        }
    }
    return divisors
}

pub fn get_factors_for(number: u64) -> Vec<u64> {
    let prime_numbers = get_prime(10000);
    get_factors_with(number, prime_numbers)
}

/// <summary>
/// This method returns a collection of factors for a given number
/// </summary>
/// <param name="number"></param>
/// <param name="primeNumbers"></param>
/// <returns></returns>
pub fn get_factors_with(number: u64, prime_numbers: Vec<u64>) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    for prime_number in &prime_numbers {
        let fnumber: f64 = number as f64;
        if fnumber.sqrt() < *prime_number as f64 { break; }
        if is_multiple(number, *prime_number) {
            factors.push(*prime_number);
            let next_number = number / prime_number;
            if prime_numbers.contains(&next_number) {
                factors.push(next_number);
            } else {
                let new_primes: Vec<u64> = prime_numbers.clone();
                factors.append(&mut get_factors_with(next_number, new_primes))
            }
            break;
        }
    }
    if factors.len() == 0 { factors.push(number); }
    factors
}
    
pub fn get_prime(max: u64) -> Vec<u64> {

    let mut prime_numbers: Vec<u64> = Vec::new();

    for number in 2..max + 1 {
        let mut not_prime: bool = false;
        let fnumber: f64 = number as f64;
        let limit: u64 = fnumber.sqrt() as u64;
        for divisor in &prime_numbers {
            // let fdivisor: f64 = *divisor as f64;
            if limit < *divisor { break; }
            if is_multiple(number, *divisor) { 
                not_prime = true;
                break;
            }
        }
        if not_prime == false { prime_numbers.push(number); }
    }
    prime_numbers
}

pub fn is_multiple(value: u64, divisor: u64) -> bool { 
    if divisor == 0 {
        false
    } else if value % divisor == 0 {
        true
    } else {
        false
    }
}

pub fn reverse_digits(number: u64) -> u64 {
    let mut result: u64 = 0;
    let mut work_number = number;

    while work_number > 0 {
        let remainder = work_number % 10;
        result = (result * 10) + remainder;
        work_number = work_number / 10;
    }

    return result;
}

pub fn series_product(numbers: Vec<u64>) -> u64 {
    let mut result: u64 = 1;
    for number in numbers {
        if number == 0 {
            result = 0;
            break;
        }
        result = result * number;
    }
    result
}

pub fn series_sum(numbers: Vec<u64>) -> u64 {
    let mut result: u64 = 0;
    for number in numbers {
        result = result + number;
    }
    result
}

pub fn square(number: u64) -> u64 { number * number }

pub fn sum_natural(number: u64) -> u64 {
    let mut result: u64 = 0;
    for sequence in 1..number + 1 { result = result + sequence; }
    result
}

pub fn sum_natural_squares(number: u64) -> u64 {
    let mut result: u64 = 0;
    for sequence in 1..number + 1 { result = result + square(sequence); }
    result
}