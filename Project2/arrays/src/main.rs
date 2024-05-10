fn main() {
    println!("========( Arrays )========");
    let mut rarr = [1,2,3,4,5,6,7,8,9,];
    println!("Our array's 0th: {}", rarr[0]);

    /*By default the arrays are immutable, hence
     * they require the mut declaration, le see*/

    rarr[0] = 3;

    println!("New element @0 is: {}", rarr[0]);

    /*now lets declare a rocksolid optimization
     * to reduce the runtime, by defining how
     * many elemts are ther in the array....*/

    let arr: [i32; 5] = [1,2,3,4,5];
    /*here we use  ^ this number to declare
     * number of elemets in an array.       */

    println!("third ele: {}", arr[3]);
}
