fn main() {
    println!("Hello, world!");
}

/*
 * Define a function that accepts two unsigned, 64 bit integers
 *  and returns an unsigned 64 bit int.
 *  The "mut" notation indicates a "mutable" variable.  the
 *      default is immutable.
 *  The assert! macro will cause the program to choke if either
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
