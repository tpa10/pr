/*
 * Program takes arguments from the cmd line and tries to convert the
 *  arguments into integers and return the gcd of all args.
 *  ex:  hello 3 6 9 -> The greatest common divisor of [3 6 9] is 3
 *
 *  To run:  cargo run <number> <...>
 *  or cargo build and execute the the binary.
 *  To run the test func:  cargo test
 */

/* 
 * Bring the "FromStr" and "env" traits into scope
 *  (rust's version of "import" -golang/java- or
 *      "include <some-library-header>" in C
 */

use std::str::FromStr;
use std::env;
fn main() {

    /*
     * Create a mutable vector to hold our cmd line args
     *   Vectors in rust are similiar to "slices" in golang
     *   and std::vector in C++.  The vector type here is 
     *   Vec<u64>, which is implicit by our usage of the vector.
     *   (i.e. the compiler "figures it out" based on the program)
     */
    let mut numbers = Vec::new();

    /*
     * Iterate through the input args, pusing them onto our vector
     * If the user gives us something that can't be translated into 
     *  a 64 bit integer, scream real loud.
     */
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    /* 
     * No valid input?  Blow them a snotgram to stderr and exit
     */
    if numbers.len() == 0 {
        eprintln!("Usage:  gcd NUMBER ...");
        std::process::exit(1);
    }

    /*
     * Range through our vector, checking for gcd
     */
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/*
 * Define a function that accepts two unsigned, 64 bit integers
 *  and returns an unsigned 64 bit int.
 *  The "mut" notation indicates a "mutable" variable.  the
 *      default is immutable.
 *  The assert! macro will cause the program to spew (panic) if either
 *      input variable is 0.
 *
 *  The function returns the Greatest Common Divisor of the two
 *      input variables.
 *  
 *  Note:  the lack of a "return" stmt is on purpose to demontrate
 *      a language feature:  IF a function body ends in a statement
 *      w/o a seimcolon, whatever that statement resolves to is used
 *      as the return value. 
 *      (Watch this, as it appears to be "idiomatic" rust!)
 */
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

/*
 * #[test] attribute will cause normal build targets to skip the following
 *  code.   (aka "cargo build")
 * If a "test" target is specified (i.e. "cargo test") the function will be
 *  compiled and run
 */

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19), 3 * 11);
}
