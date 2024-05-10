/*here we will import the library for user input
 * this is called the io-module.   */

use std::io;

/*here the std refers to the standard library of 
 * rust programming language, just like numpy or the
 * infamous pandas. */
/*here the io is the input-output "Module" include in
 * the standard("std") library of rust. */


fn main() {
    println!("Hello, world!");
    
    /*the userinput function is not included in the
     * standard library or the prelude of the rust
     * hence we need to explicitly import it.    */
    
    let mut input = String::new(); /* "::" is called as the path seperator operator */
    
    println!("Enter your name here: "); 

    io::stdin().read_line(&mut input).expect("failed to read line");

    /*if you forget &mut it'll use a copy of var, and wont edit it irl
     * and '&' here is a refrence to another var and can be used as 
     * '&input' as in our case here. 
     * expect is catching any errors that may occour. */

    println!("\nWelcome, {}", input);


    /*here in rust a crate is a package or a library for 
     * rust programs, just a remider*/

}
