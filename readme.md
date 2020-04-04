# ProjectEuler
Project Euler was started by Colin Hughes (a.k.a. euler) in October 2001 as a sub-section on mathschallenge.net. Who could have known how popular these types of problems would turn out to be? Since then the membership has continued to grow and Project Euler moved to its own domain in 2006.

## What is this?
It is important to code every day to keep our skills sharp. I have created this repository for my solutions to Project Euler challenges. This particular repository contains my solutions written in Rust. I have similar repositories for Python,
Java, C#, Scala, and C++.

## Structure of this code
This application runs as a console application. It is build for Rust running on Ubuntu and was developed on the WSL. The Rust language uses *libraries* and *modules* to organize code. This app have one library and three modules. These are:

- ** src ** this is the library. It contains the main program and all of the application modules.
- ** euler_library ** this is the parent folder for the solution's class libraries.
- ** problems ** this module contains the solutions to the problems.
- ** tests ** these are the unit tests for the solution.

## Main Program
The main program prompts the user for a problem number, then runs the appropriate problem and reprompts the user. To end the program the user types "q" and presses enter.

The main program calls functions to validate the user's input, and then uses a factory pattern to obtain the class for the selected problem. Run times are tracked and reported by the main program.

## euler_library
The EulerLibrary is the SDK for the solutions. The library is composed to two modules: math_library and problems. The utilities functions are contained within this modules mod.rs source file.

### math_Library
The MathLibrary contains the algorithms needed to solve the problems. There are two source files in this namespace, these are.

## Problems
The Problems namespace contains the solution to the Project Euler problems. Each problem has its' own source file. In addition, the factory function (problem_factory) is contained within the problems' mod.rs. The factory accepts an integer problem number and returns the appropriate class to solve that problem. The problems are based on the interface *IEulerSolution*.