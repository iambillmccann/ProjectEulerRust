extern crate stopwatch;
extern crate project_euler;

use stopwatch::{Stopwatch};
use std::io;
use project_euler::euler_library::problems::*;
use project_euler::euler_library::utilities;

const LAST_PROBLEM: i32 = 8;

/// getUserInput is a simple method for reading from the console.
/// Returns: A string that represents the user's input.
fn get_user_input() -> i32 {
    let mut problem_number = String::new();

    println!("\nWhat problem shall I run? Or type 'Q' to quit. ");
    io::stdin().read_line(&mut problem_number)
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
    if user_input.trim().to_ascii_uppercase() == "Q" { return result }

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
        println!("\nI have only completed problems 1 through {}", LAST_PROBLEM);
        result = get_user_input();
    };

    result

}

/// This is the main program
fn main() {

    let mut problem: i32 = get_user_input();

    while problem > 0 {

        let mut total_time = Stopwatch::start_new();
        let result: String = problem_factory::get_solution(problem);
        total_time.stop();

        println!("-----------------------------------------------------------------");
        println!("Solution to problem {} = {}", problem, result);
        println!("Execution time was {}", utilities::format_milliseconds(total_time.elapsed_ms()));
        println!("-----------------------------------------------------------------");

        problem = get_user_input();

    }

}
