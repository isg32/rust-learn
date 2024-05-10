use std::io;

fn main() {
    /*this one is going to be basic arithmetics and type casting
     * in Rust. */

    let x: f32 = 5.0;  // u8 range is 0-255
                       // if you add a value over 255 it'll overflow
                       // so use u16 or u32 for longer digits.

    let y: i32 = 10; // i8 range is -128 - 127 
    
    /*To avoid overflows, we use float value for unrestriced 
     * addition. */

    let z = (y as f32) + x;
    println!("value of x + y: {}", z);
    
    /*Now using user input with arithmetic. */
    
    let mut intpt = String::new();
    println!("Enter A number: ");
    io::stdin().read_line(&mut intpt).expect("can't read lines bruv");
    
    /*converting the string input to intiger@64, also
     * using parse to enable convert,
     * using unwrap as some issue handler or shit. */

    let intinput: i64 = intpt.trim().parse().unwrap();
    
    /*adding +2 to check if the parsed input is what
     * we expected or needed. */

    println!("{}", intinput + 2);

}
