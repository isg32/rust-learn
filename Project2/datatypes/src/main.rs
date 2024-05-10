fn main() {
    println!("1. Primitive Data-types");
    
    /*signed intiger - positive&negetive*/
    let x: i32 = 1011;

    /*unsigned intiger - only positive*/
    let f: u32 = 1250;

    /*Floating values*/
    let float: f32 = 10.9;
    let floatx: f64 = 19.1;

    /*Bools*/
    let tof: bool = true;
    let fot: bool = false;

    /*char type*/
    let lettah: char = 's';
    /*this can basically store anything with*/
    /*one letter let it be number brackets  */
    /*or be it a character.                 */
    

    /*Now cums the compound datatypes*/

    let tups: (i32, bool, char)  = (1, true, 's');

    /*Now printing the compound datas can be */
    /*tricky as em can't be printed together */

    println!("this is the first index of a compound: {}",tups.0);

    /*using the .[index_no.] helps us print  */
    /*em individually. hence no errors, this */
    /*is how its supposed to be done.        */

    /*now by default compound datatypes are  */
    /*immutable but adding an explicit inst. */
    /*ain't gonna hurt your mom.             */

    let mut tupf: (i8, char, bool) = (0, '^', false);

    println!("second indes of mutable tuple: {}", tupf.1);

    let tupf.1 = 's';
    /*it not my issue the above line gives   */
    /*error, its just the rustcompiler gaying*/
    /*out, try on any other ver.             */
    println!("second indes after edit: {}", tupf.1);

}
