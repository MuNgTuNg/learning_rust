

// ///documentation comment. println is a macro not a function call


// pub fn hello(string1 : &str){
//     println!("{}", string1);
// }

#[allow(unused_assignments)]
#[allow(dead_code)]
pub fn variables_main(){
    ////this won't compile as x is immutable
    //let x = 5;
    //println!("The value {x} is: x");
    //x = 6;
    //println!("The value {x} is: x");

    //this will.
    let mut x = 5;
    println!("The value {x} is: x");
    x = 6;
    println!("The value {x} is: x");

    //const needs to have type annotation, rust can evaluate operations at compile time
    const ONE_HUNDRED: i32 = 10 * 10;
    print!("one hundred = {ONE_HUNDRED}\n");

    //the second variable "shadows" the first one. does not throw redeclaration. 
    let x = 10;
    let x =  x - 100;
    print!("x = {x}\n");

    //shadowing allows mutation of variable's type, without the need for multiple variables
    //such as spaces for the string and num_spaces for the length.
    let spaces = "    ";
    let spaces = spaces.len();
    print!("Spaces = {spaces}\n");

    //this won't  work though as we can't mutate the variable's type directly.
    // let mut spaces = "     ";
    // spaces = spaces.len();

    
    //rust's compiler panics at overflows at runtime in debug mode but not in release mode.
    let mut x :i8  = 127;
    x = x.wrapping_add(12);
    //x += 12;
    print!("x = {x}");


   //TUPLES
    // let tuple: (f32, char, &str) = (5.5, 'c', "Hello");
    let tuple = (5.5, 'c', "Hello");
    let (number,letter, mut string) = tuple; //access elements using pattern matching tuple deconstruction
    string = "smello";
    print!("{number},{letter},{string}\n"); 
    print!("{0}\n", tuple.1); //or access via index \



   //ARRAYS
    let arr = [1,2,3,4,5,6,7]; //implicit type and size
    let _arr2: [i32; 2] = [22,33]; //explicit type and size
    let _arr3 = [2;20]; //twenty number twos

    //range based for loop like python
    for i in 0..arr.len(){
        print!("{}",arr[i]);
    }
    
    
}


// /*
// scalar types
//     -integer
//         - i8, i16, i32, i64, i128, isize (isize depends on architecture 32/64 bit)
//     -floating points
//         - f32 <- single, f64 <- double
//     -bool
//     -char

// */

// //CONTINUE HERE 
// //https://doc.rust-lang.org/book/ch03-03-how-functions-work.html