fn main() {

    /*Immutable variables*/
    let x = 4;
    let y = 9;
    println!("x + y = {} ",x+y);
    
    /* Mutable variables */
    let f = 4;
    println!("Value of f righ now : {}",f);
    let f = 81;
    println!("Value of f after : {}",f);
    
    /* giving an abstarction */
    println!("The value of x outside abs: {}",x);
    {
        let x = 10;
        println!("value of x in abs: {}",x);
    }
    println!("again outside abs, x: {}",x);
}
