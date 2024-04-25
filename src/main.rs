extern crate project_euler;
// extern crate stopwatch;

use project_euler::euler_library::problems::*;
// use project_euler::euler_library::utilities;
use std::io;
use std::time::Instant;

const LAST_PROBLEM: i32 = 13;

/// getUserInput is a simple method for reading from the console.
/// Returns: A string that represents the user's input.
fn get_user_input() -> i32 {
    let mut problem_number = String::new();

    println!("\nWhat problem shall I run? Or type 'Q' to quit. ");
    io::stdin()
        .read_line(&mut problem_number)
        .expect("Oops. Something went wrong when getting the problem number.");

    check_user_input(problem_number)
}

/// checkUserInput will validate the user's input. This is a very redimentary
/// validation method. It checks three things ...
/// 1. Checks for the letter "Q"
/// 2. Checks for an integer
/// 3. Checks that the integer is positive
/// 4. Checks that the integer is no greater than the last problem solved.
/// These checks assume that problems are solved in order.
/// Note: If the user enters "Q", a minus one is returned to the caller.
/// Parameter: "userInput", A string entered by the user.
/// Returns: A valid integer (i32) problem number.
fn check_user_input(user_input: String) -> i32 {
    let mut result: i32 = -1;
    if user_input.trim().to_ascii_uppercase() == "Q" {
        return result;
    }

    result = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\nSorry but I did not understand that. Please type the problem number or Q to quit.");
            get_user_input()
        }
    };

    if result < 0 {
        println!("\nBTW, problem numbers are positive integers.");
        result = get_user_input();
    } else if result > LAST_PROBLEM {
        println!(
            "\nI have only completed problems 1 through {}",
            LAST_PROBLEM
        );
        result = get_user_input();
    };

    result
}

/// run_problems is the main program logic, it uses a recursive loop to get input and
/// select a problem number.
///
/// Args
///    problem_number  The Project Euler problem number chosen by the user
///
/// Returns
///    The value zero
fn run_problem(problem_number: i32) -> i32 {
    if problem_number < 1 {
        return 0;
    }

    let start = Instant::now();
    let result: String = problem_factory::get_solution(problem_number);
    let duration = start.elapsed();

    println!("-----------------------------------------------------------------");
    println!("Solution to problem {} = {}", problem_number, result);
    println!(
        "Execution time was {}",
        duration.as_millis() // Converts and displays the time as milliseconds
    );
    println!("-----------------------------------------------------------------");

    run_problem(get_user_input())
}

/// This is the main program
fn main() {
    println!("Welcome to Project Euler in Rust.");
    run_problem(get_user_input());
    println!("Goodbye");
}
